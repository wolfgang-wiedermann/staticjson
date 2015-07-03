//
// Type-Definition for Sampling
//

type AllTypesEntity {
  // String Samples
  m_string:string;
  m_string_array:string[];
  m_string_mandatory:string(mandatory="true");
  m_string_obligatory:string(mandatory="false");

  // Signed Int Samples
  m_int:int;
  m_int_array:int[];
  m_int_mandatory:int(mandatory="true");
  m_int_mandatory_array:int[](mandatory="true");
}
