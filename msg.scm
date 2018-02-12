(name "msg")
(version "0.1.0")

(type req record
  (fields
    (field seq  u32)
    (field data u64)))

(type res record
  (fields
    (field seq  u32)
    (field data u64)))

(type transaction union
  (fields
    (field req  req)
    (field res  res)))
