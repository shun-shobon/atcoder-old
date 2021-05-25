import           Control.Monad

main = do
  n    <- readLn
  txys <- replicateM n $ map read . words <$> getLine
  putStrLn $ if analyze ([0, 0, 0] : txys) then "Yes" else "No"

analyze [a, b]       = reachable a b
analyze (a : b : xs) = reachable a b && analyze (b : xs)

reachable [t1, x1, y1] [t2, x2, y2] =
  (dt >= dx + dy) && ((dt - dx - dy) `mod` 2 == 0)
 where
  dx = abs (x2 - x1)
  dy = abs (y2 - y1)
  dt = t2 - t1
