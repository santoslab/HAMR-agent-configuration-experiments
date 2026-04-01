package GumboLib {
  private import HAMR::*;
  private import SNG_Data_Model::*;
    
  language "GUMBO" /*{
    library

    functions
      def clampedPayloadLowerBound(): Base_Types::Integer_32 := 0[i32];   
      def clampedPayloadUpperBound(): Base_Types::Integer_32 := 100[i32];   

      def clampedPayload(m: SNG_Data_Model::Message): Base_Types::Boolean :=
          (clampedPayloadLowerBound() <= m.payload) and (m.payload <= clampedPayloadUpperBound());

      def allowedSecurityLevel(m: SNG_Data_Model::Message): Base_Types::Boolean :=
          (m.security_level == SNG_Data_Model::SecurityLevel::Restricted) or 
          (m.security_level == SNG_Data_Model::SecurityLevel::Public);

      def equalSecurityLevel(m1: SNG_Data_Model::Message,
                             m2: SNG_Data_Model::Message): Base_Types::Boolean :=
          (m1.security_level == m2.security_level);

      def equalPayload(m1: SNG_Data_Model::Message,
                       m2: SNG_Data_Model::Message): Base_Types::Boolean :=
          (m1.payload == m2.payload);

      def equalMessage(m1: SNG_Data_Model::Message,
                       m2: SNG_Data_Model::Message): Base_Types::Boolean :=
          equalSecurityLevel(m1,m2) and equalPayload(m1,m2);

  }*/
}