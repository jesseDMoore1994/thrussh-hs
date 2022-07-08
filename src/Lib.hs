{-# LANGUAGE ForeignFunctionInterface #-}

module Lib where

import Curryrs.Types
import Foreign
import Foreign.C.Types
import Foreign.Ptr
import Foreign.Ptr

type CData = Ptr ()

data Runtime
data Session

foreign import ccall "create_async_runtime" create_async_runtime :: IO (Ptr Runtime) 
foreign import ccall "release_async_runtime" release_async_runtime :: Ptr Runtime -> IO ()
foreign import ccall "create_session" create_session :: Ptr Runtime -> IO (Ptr Session) 
foreign import ccall "destroy_session" destroy_session :: Ptr Session -> IO ()
foreign import ccall "callex" callex :: Ptr Runtime -> Ptr Session -> IO ()

foreign import ccall "create_data_c" create_data_c :: IO CData
foreign import ccall "destroy_data_c" destroy_data_c :: CData -> IO ()
foreign import ccall "print_data_c" print_data_c :: CData -> IO ()
foreign import ccall "hello" hello :: ()
foreign import ccall "double" double :: I64 -> I64
foreign import ccall "square" square :: I64 -> I64
foreign import ccall "cube" cube :: I64 -> I64

squarePlus1 = (+) 1 $ square 4

someFunc :: IO ()
someFunc = do
  cdata <- create_data_c
  print_data_c cdata
  destroy_data_c cdata

  runtime <- create_async_runtime
  session <- create_session runtime
  callex runtime session
  return ()

