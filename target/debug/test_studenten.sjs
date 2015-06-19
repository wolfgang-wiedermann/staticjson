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
  geburstdatum:date;
  geburtsort:OrtEntity;
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
