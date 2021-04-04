pub fn do_a_struct() {
   #[derive(Debug)]
   struct MyStruct {
      foo: String,
      bar: bool,
      tup: (u32, u32)
   }

   let a_struct = MyStruct {
      foo: String::new(),
      bar: false,
      tup: (7, 8)
   };
   println!("I made a struct: {:?}.", a_struct.foo);
}

pub fn generics() {
   let number_list = vec![18, 30, 11, 19];
   let result = largest(&number_list);
   println!("largest number is: {}.", result)
}

pub fn largest<T: PartialOrd>(list: &[T]) -> &T {
   let mut largest = &list[0];
   for item in list {
       if item > largest {
           largest = item;
       }
   }
   largest
}
