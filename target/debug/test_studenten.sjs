// Ein schnell geschriebenes Test-Beispiel:

type StudentEntity (
    java-package="de.ww.entities",
    jpa-entity="true",
    jpa-table="tbl_student",
    cs-namespace="Test.Model",
    ef-table="tbl_student") {
    
  mtknr_id:decimal(maxlen="10", decimals="0", mandatory="true");
  mtknr_id2:int[];
  mtknr_id3:uint(
    jpa-id="true",
    jpa-generated-value="true",
    ef-id="true",
    ef-generated-value="true"
  );
  mtknr_id4:long;
  mtknr_id5:ulong;
  string_array:string[];
  vorname:string(mandatory="true");
  nachname:string(mandatory="true");
  testdings:string(mandatory="false");
  geburstdatum:date;
  geburtsort:OrtEntity;
  wohnorte:OrtEntity[];
  ganzzahl:int; 
  custom_order_id:decimal(jpa-transient="true");
}

type OrtEntity (
    java-package="de.ww.entities",
    jpa-entity="true",
    jpa-table="tbl_ort",
    cs-namespace="Test.Model") {
    
  postleitzahl:string(
    jpa-column="plz",
    jpa-id="true",
    maxlen="5", 
    minlen="5",  
    mandatory="true"
  );
  ortsname:string(
    jpa-column="wohnort",
    maxlen="50"
  );
  landkreis_id:int(
    jpa-column="landkreis_id"
  );
}

type OtherPackageEntity (
    java-package="de.ww.entities2",
    cs-namespace="Test.Model") {
  test:string;
  test2:string;
  test3:OrtEntity;
}

//
// First simple interface sample
//
interface StudentRepository(
    java-package="de.ww.interfaces",
    js-namespace="de.ww.proxy",
    cs-namespace="Test.Controller",
    path="/repos") {

  // Get-Method for Students
  // "method" is a mandatory param per function 
  //   it definies the HTTP-Method to use for service call
  getStudentById(id:int(path-param="id")) -> StudentEntity {
    method="GET",
    path="/student/{id}"
  }
  
  // Find-Method for Students
  // "method" is a mandatory param per function 
  //   it definies the HTTP-Method to use for service call
  // "path" is mandatory if no Interface-Parameter path is available
  findStudents(name:string(query-param="name"), 
              vorname:string(query-param="vorname")) -> StudentEntity[] {
    method="GET",
    path="/student"
  }
  
  // Find-Method for Studends by OrtEntity
  // helps to check working include functions
  // HINT: Pracitically this method is bullshit because GET can not handle an object payload
  findStudentsByOrt(ort:OrtEntity) -> StudentEntity[] {
    method="GET",
    path="/student/byOrt"
  }
  
  // Method to create a new student object
  createStudent(s:StudentEntity) -> StudentEntity {
    method="POST",
    path="/student"
  }
  
  // Method to create a set of new student objects
  createStudents(s:StudentEntity[]) -> StudentEntity {
    method="POST",
    path="/students"
  }
  
    // Method to create a set of new student objects
  createStudents2(s:StudentEntity[](serialized-as="text/xml")) -> StudentEntity[] {
    method="POST",
    path="/students"
  }
  
  // Method to update a new student object
  updateStudent(s:StudentEntity, id:int(path-param="id")) {
    method="PUT",
    path="/student/{id}"
  }
  
  // Sample for a DELETE method with path param
  removeStudent(id:int(path-param="id")) {
    method="DELETE",
    path="/student/{id}"
  }
  
  // POST with Query-Params
  createDings(id:int(query-param="id")) -> StudentEntity {
    method="POST",
    path="/dings"
  }
}

//
// Second simple interface sample with interface parameters
//
interface OrtRepository(
    java-package="de.ww.interfaces",
    cs-namespace="Test.Controller",
    pattern="Repository", 
    path="/repos/ort") {
    
  // Get-Method for Ort-Objects
  // method is a mandatory param per function 
  // it definies the HTTP-Method to use for service call
  getOrtById(id:int(path-param="id")) -> OrtEntity {
    method="GET",
    path="/{id}"
  }  
}

//
// Sample: Interface with empty param list (makes no sence but is also supported)
//
interface WithoutParamsAndFunctions() {

  // No function definitions inside
  
}