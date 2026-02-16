/*============================================================
    P l a t f o r m

    Define computational resources and other attributes
    associated with a platform.
  ============================================================*/

package Platform {
    private import HAMR::*;

    //===============================================================================
    //
    //  SNG Processor
    //
    //===============================================================================
    part def SNG_Processor :> Processor {
        attribute :>> Frame_Period = 1 [s];
        attribute :>> Clock_Period = 1 [s];
    }
}
