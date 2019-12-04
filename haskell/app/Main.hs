module Main where

getFuel1 :: Int -> Int
getFuel1 n = n `div` 3 - 2

getFuel2 :: Int -> Int
getFuel2 n
  | (n `div` 3 - 2) <= 0 = 0
  | n > 0 = (n `div` 3 - 2) + getFuel2 (n `div` 3 - 2)

main :: IO ()
main = do
  input <- readFile "../src/bin/day_01_data.txt"
  print $ sum . map (getFuel1 . read :: String -> Int) $ lines input
  print $ sum . map (getFuel2 . read :: String -> Int) $ lines input
  let l = map (read::String->Int ) $lines input
  let b = map (+ 1) l
  print b
