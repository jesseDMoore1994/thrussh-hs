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

foreign import ccall "create_session" create_session :: IO (Ptr Session) 
foreign import ccall "destroy_session" destroy_session :: Ptr Session -> IO ()
foreign import ccall "callex" callex :: Ptr Session -> IO ()

foreign import ccall "hello" hello :: ()
foreign import ccall "double" double :: I64 -> I64
foreign import ccall "square" square :: I64 -> I64
foreign import ccall "cube" cube :: I64 -> I64

someFunc :: IO ()
someFunc = do
  session <- create_session
  callex session
  return ()

