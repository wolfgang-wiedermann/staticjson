// This is the sample staticjson code for the README placed on github

type Customer (
        java-package="de.ww.sample.entities",
        jpa-entity="true",
        jpa-table="tbl_customer",
        js-namespace="de.ww.sample.entities"
    ) {
  
  customer_id:int(
    jpa-id="true", 
    jpa-generate-value="true"
  );

  prename:string(maxlen="50");
  surname:string(maxlen="50");
}

interface CustomerRepository (
        java-package="de.ww.sample",
        js-namespace="de.ww.sample.proxy",
        path="/api"
    ) {

  // Get a Customer by its id
  getCustomerById(id:int(path-param="id")) -> Customer {
    method="GET",
    path="/customer/{id}"
  }

  // Find a customer by its pre- or surname
  findCustomer(prename:string(query-param="prename"),
               surname:string(query-param="surname")) -> customer[] {
    method="GET",
    path="/customer"
  }

  // Create a new customer and return it with its server generated id
  createCustomer(c:Customer) -> Customer {
    method="POST",
    path="/customer"
  }

  // Deletes a customer by its id and returns the id at success
  deleteCustomer(id:int(path-param="id")) -> int {
    method="DELETE",
    path="/customer/{id}"
  }
}
