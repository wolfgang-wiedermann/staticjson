// Ein schnell geschriebenes Test-Beispiel:

// Was auch noch fehlt sind die Typ-Parameter
// also: type Student(typparametername="typparameterwert") { ... }
type StudentEntity {
  mtknr_id:decimal(maxlen="10", decimals="0", mandatory="true");
  vorname:string(mandatory="true");

  // TODO: Sonderzeichen in Typnamen, Attributnamen und Parameternamen müssen unterbunden werden
  //       erlaubt sein sollten: [a-zA-Z0-9_-]
  nachname:string(mandatory="true");
  geburstdatum:date;
  geburtsort:OrtEntity; 
  custom_order_id:decimal;
}

type OrtEntity {
  // TODO: Hier gibts noch das folgende Problem
  // -> ein Ort kann mehrere Postleitzahlen haben...
  postleitzahl:decimal(
    maxlen="5", 
    minlen="5", 
    decimals="0", 
    mandatory="true"
  );
  ortsname:string;
  landkreis_id:int;
}
