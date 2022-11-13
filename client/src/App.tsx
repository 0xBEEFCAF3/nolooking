import * as React from "react"
import {
  ChakraProvider,
  Box,
  Text,
  VStack,
  Grid,
  Heading,
  theme,
  Flex,
  Input,
  FormControl,
  FormLabel,
  Button,
  GridItem,
  IconButton,
} from "@chakra-ui/react"
import { AddIcon, MinusIcon } from '@chakra-ui/icons'

import { ColorModeSwitcher } from "./ColorModeSwitcher"
import { Logo } from "./Logo"

export const App = () => {
  type Node = {address: string, amount: string};
  const [nodes, setNodes] = React.useState<Node[]>([{address: '', amount:''}]) 

  const setNodeAmount = (index:number, amount: string ): void => {
    setNodes(nodes.map((node, i) => i === index ? {...node, amount} as Node : node ));
  }

  const setNodeAddress = (index:number, address: string ): void => {
    setNodes(nodes.map((node, i) => i === index ? {...node, address} as Node : node ));
  }

  const addNode = () => {
    setNodes([...nodes, {address: '', amount: ''}])
  }

  const removeNode = (index: number) => {
    setNodes(nodes.filter((_node, i) => i !== index))
  }

  return (
    <ChakraProvider theme={theme}>
    <Box textAlign="center" p={10}>
      <Flex textAlign="center" justifyContent="center">
        <Logo h="10vmin" pointerEvents="none"  />
        </Flex>
      <Flex textAlign="center" justifyContent="center">
        <Heading size='4xl' noOfLines={1}>&nbsp;Lightning PayJoin&nbsp;</Heading>
        <ColorModeSwitcher justifySelf="flex-end" />
      </Flex>
      {/* <Heading noOfLines={1}>Alpha [exp erimental] | Avoid sp👀ks</Heading> */}
      <Heading size='2xl' noOfLines={1}>Queue batches of lightning channels to open in a single transaction</Heading> 
    </Box>
    <VStack>
      <Box p={8} borderWidth={1} borderRadius={8} boxShadow="lg" minW="800" maxW="200">
        <Heading textAlign="center" pb={10}>Config</Heading>
        <Box my={4} textAlign="left">
          <form>
            <FormControl>
              <FormLabel>Anchor Payment</FormLabel>
              <Input type="text" />
            </FormControl>
            <FormControl mt={6}>
              <FormLabel>On-Chain Fee</FormLabel>
              <Input type="text" placeholder="sat/vb" />
            </FormControl>
          </form>
          </Box>
      </Box>
      <Box p={8} borderWidth={1} borderRadius={8} boxShadow="lg" minW="800" maxW="200">
        <Heading textAlign="center" pb={10}>Queue Channels to Open</Heading>
        
        <Grid templateColumns='repeat(3, 1fr)' gap={6}>
        <GridItem w='100%' h='10'> <Text textAlign="center">Destination Node</Text></GridItem>
        <GridItem w='100%' h='10'> <Text textAlign="center">Channel Capacity (sats)</Text></GridItem>
        </Grid>
        <Grid templateColumns='repeat(3, 1fr)' gap={6}>
        {nodes.map((node, i) => (
            <>
          <GridItem w='100%' h='10'> <Input type="text" placeholder="node@host:port" value={node.address} onChange={(e) => setNodeAddress(i, e.currentTarget.value)}/></GridItem>
          <GridItem w='100%' h='10'> <Input type="text" placeholder="20000" value={node.amount} onChange={(e) => setNodeAmount(i, e.currentTarget.value)} /></GridItem>
          <IconButton
          w={25}
            variant='outline'
            colorScheme='teal'
            aria-label='Call Sage'
            fontSize='20px'
            icon={<MinusIcon />}
            onClick={() => removeNode(i) }
            disabled={nodes.length <= 1}
          />          
          </>
        ))}
          
        </Grid>
        <IconButton
            variant='outline'
            colorScheme='teal'
            aria-label='Call Sage'
            fontSize='20px'
            mt={20}
            icon={<AddIcon />}
            onClick={addNode}
            disabled={nodes.length > 10}

          />
         
      </Box>
      <Button colorScheme='teal' size='lg'>
        Generate Funding Request
      </Button>
    </VStack>
  </ChakraProvider>
)
}
 