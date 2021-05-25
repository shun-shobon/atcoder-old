main = do
  s <- getLine
  putStrLn $ if f (reverse s) then "YES" else "NO"

f "" = True
f ('m' : 'a' : 'e' : 'r' : 'd' : k) = f k
f ('r' : 'e' : 'm' : 'a' : 'e' : 'r' : 'd' : k) = f k
f ('e' : 's' : 'a' : 'r' : 'e' : k) = f k
f ('r' : 'e' : 's' : 'a' : 'r' : 'e' : k) = f k
f _  = False
