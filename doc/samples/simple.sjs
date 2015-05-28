//
// A simple example to show how StaticJSON works
// 

type Customer (allow_undefined_fieldnames="true") {
  // Inner-Type-Comments
  customer_number:decimal(
    maxlen="10", 
    decimals="0"
  ); // Inner-Type-Comment2
  prename:string(maxlen="50");
  surname:string(maxlen="50");
  addresses:Address[];
  // Comments between ( and ) of an Attribute are not allowed!
}

type Address {
  street:string;
  housenumber:string(
    maxlen="10", 
    regex="[0-9]+"
  );
  postcode:string(
    maxlen="10",
    regex="[A-Z]-[0-9]+"
  );
  city:string(maxlen="50");
  database_id:int;
}
