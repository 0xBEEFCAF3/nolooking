use std::io::{stdout, Write};

use bitcoin::{Address, Amount};
use hyper::body::HttpBody;
use hyper::client::HttpConnector;
use hyper::{Body, Client, Method, Uri};
use hyper::http::{Request, Response};
use hyper::http::Error;
use hyper_tls::{native_tls, HttpsConnector};
use url::Url;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZeroFeeChannelOpenResponse {
    pub invoiceId: String,
    pub checkoutLink: String,
    pub lnurlChannel: String
}


// TODO return result here
async fn body_to_string(body: Body) -> String {
    let body_bytes = hyper::body::to_bytes(body).await.unwrap();
    String::from_utf8(body_bytes.to_vec()).unwrap()
}

pub async fn schedule_inbound_openings(node_id: String, amount: Amount) -> Result<& 'static str, hyper::http::Error> {
    let mut url = Url::parse("https://api.zerofeerouting.com/v1/channel/new/").unwrap();
    url.query_pairs_mut().append_pair("peer", node_id.as_str());
    url.query_pairs_mut().append_pair("funding_local", amount.to_string().as_str());

    let https = HttpsConnector::new();
    let client = Client::builder()
        .build::<_, hyper::Body>(https);

    let amount_msat = amount.as_sat() * 1000;
    let uri: Uri = format!("https://api.zerofeerouting.com/v1/channel/new/?peer={}&funding_local={}", node_id.as_str(), amount_msat.to_string()).parse().unwrap();
    let req = Request::post(uri).body(Body::empty()).unwrap();
   
    let resp = client.request(req).await.unwrap();
    println!("{:?}", resp);

    let body_str = body_to_string(resp.into_body()).await;
    let zero_fee_response: ZeroFeeChannelOpenResponse = serde_json::from_str(&body_str).unwrap();

    println!("{:?}", zero_fee_response);
    let https = HttpsConnector::new();
 
    let client = Client::builder()
    .http2_only(true)
    .build::<_, hyper::Body>(https);
    // https://btcpay.zerofeerouting.com/i/GVBJytJ3i3GP4E93SaCpB6/status?invoiceId=GVBJytJ3i3GP4E93SaCpB6&paymentMethodId=BTC&_=1666536479376
    // https://btcpay.zerofeerouting.com/i/HsAgVx8kMHudBEnfCituko/status?invoiceId=HsAgVx8kMHudBEnfCituko&paymentMethodId=BTC&_=1666536479376
    let btc_pay_uri = format!("{}/status?paymentMethodId=BTC", zero_fee_response.checkoutLink);
    let btc_pay_uri_parse: Uri = btc_pay_uri.parse().unwrap();
    println!("prase uri:: : {:?}", btc_pay_uri_parse);

    let res = client.get(btc_pay_uri.parse().unwrap()).await.unwrap();


    println!("{:?}", res);
    Ok("")
}


#[tokio::test]
async fn get_response() {
    let amount = Amount::from_sat(100_000);
    let result = schedule_inbound_openings("038b177029ed7aa196cf038e1e264be2c35e02e1397d5975ac6a329cc7cdd3a1e6@rxc7r65xzbb5mz6cf6mtgduiu54ht2yk4r7q6yl6gk7xfpqd2jm6u4id.onion:9735".to_string(), amount).await;


    assert!(result.is_ok())
}




