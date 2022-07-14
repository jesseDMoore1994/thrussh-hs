{-# LANGUAGE ForeignFunctionInterface #-}

module Lib where

import Curryrs.Types
import Foreign
import Foreign.C.Types
import Foreign.C.String (CString(..), newCString)
import Foreign.Ptr
import Foreign.Ptr
import System.IO

type CData = Ptr ()

data Runtime
data Session

foreign import ccall "create_session"
    create_session :: CString -> CString -> CString -> IO (Ptr Session)
foreign import ccall "destroy_session"
    destroy_session :: Ptr Session -> IO ()
foreign import ccall "callex"
    callex :: Ptr Session -> CString -> IO ()

someFunc :: IO ()
someFunc = do
  target <- newCString "127.0.0.1:22"
  username <- newCString "test"
  putStrLn "Enter your password:"
  hSetEcho stdin False
  password <- newCString =<< getLine
  hSetEcho stdin True
  session <- create_session target username password
  putStrLn "Enter the remote command:"
  command <- newCString =<< getLine
  callex session command
  destroy_session session
  return ()

