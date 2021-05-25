main = do
  [a, b] <- map read . words <$> getLine

  putStrLn $ if a * b `mod` 2 == 0 then "Even" else "Odd"
