main = do
  _  <- getLine
  as <- map read . words <$> getLine
  print $ minimum $ map f as

f x = if x `mod` 2 == 0 then 1 + f (x `div` 2) else 0
