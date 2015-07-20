// Ein schnell geschriebenes Test-Beispiel:

type StudentEntity (type_param="1234") {
  mtknr_id:decimal(maxlen="10", decimals="0", mandatory="true");
  mtknr_id2:int[];
  mtknr_id3:uint;
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
  custom_order_id:decimal;
}

type OrtEntity {
  postleitzahl:decimal(
    maxlen="5", 
    minlen="5", 
    decimals="0", 
    mandatory="true"
  );
  ortsname:string;
  landkreis_id:int;
}

//
// First simple interface sample
//
interface StudentRepository {

  // Get-Method for Students
  // "method" is a mandatory param per function 
  //   it definies the HTTP-Method to use for service call
  getStudentById(id:int) -> StudentEntity {
    method="GET",
    path-params="id",
    path="/repos/student/{id}"
  }
  
  // Find-Method for Students
  // "method" is a mandatory param per function 
  //   it definies the HTTP-Method to use for service call
  // "path" is mandatory if no Interface-Parameter path is available
  findStudent(name:string, vorname:string) -> StudentEntity[] {
    method="GET",
    query-params="name,vorname",
    path="/repos/student"
  }
}

//
// Second simple interface sample with interface parameters
//
interface OrtRepository(
    pattern="Repository", 
    path="/repos/ort") {
    
  // Get-Method for Ort-Objects
  // method is a mandatory param per function 
  // it definies the HTTP-Method to use for service call
  getOrtById(id:int) -> OrtEntity {
    method="GET",
    path-params="id",
    path="{id}"
  }
}