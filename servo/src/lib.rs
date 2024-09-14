pub mod servo{
   pub static Servo_VERSION:i32 = 2;
   pub static MIN_PULSE_WIDTH:i32 = 544;
   pub static MAX_PULSE_WIDTH:i32 = 2400;
   pub static DEFAULT_PULSE_WIDTH:i32 = 20000;

   pub static SERVOS_PER_TIMER:i32 = 12;
   pub static MAC_SERVOS:i32 = SERVOS_PER_TIMER;

   pub static INVALID_SERVO:i32 = 255;

   pub struct ServoPin_t  {
        pub nbr:i32,
        pub isActive:i32
   }

   pub struct servo_t{
        pub pin:ServoPin_t,
        pub ticks:u32
   }

}


