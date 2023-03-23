pub struct CPU {
    program_counter: u16,
    stack_pointer: u32,
    accumulator: u8,
    register_x: u32,
    register_y: u32,
    status: u8
}
// STATUS = N V 0 B D I Z C
impl CPU {
    pub fn new() -> Self {
        CPU {
            program_counter: 0,
            stack_pointer: 0,
            accumulator: 0,
            register_x: 0,
            register_y: 0,
            status: 0
        }
    }

    pub fn clc_clear_carry_flag(&mut self) {
        self.status = self.status & 0b1111_1110;
    }

    pub fn sec_set_carry_flag(&mut self) {
        self.status = self.status | 0b0000_0001;
    }

    pub fn set_zero_flag(&mut self) {
        self.status = self.status | 0b0000_0010;
    }

    pub fn set_overflow_flag(&mut self) {
        self.status = self.status | 0b0100_0000;
    }    

    pub fn remove_overflow_flag(&mut self) {
        self.status = self.status & 0b1011_1111;
    }   

    pub fn set_negative_flag(&mut self) {
        self.status = self.status | 0b1000_0000;
    }    

    pub fn adc_add_with_carry(&mut self, value: u8) {
        let mut sum = value as u16 + self.accumulator as u16;
        if self.status & 0b0000_0001 !=0 {
            sum += 1;
            self.clc_clear_carry_flag();
        }

        if sum == 0 {
            self.set_zero_flag();
        }

        if sum > 0xff {
            self.sec_set_carry_flag();
        }

        let result = sum as u8;

        if ((value  ^ result ) & (self.accumulator ^ result)) & 0x80 != 0 {
            self.set_overflow_flag();
        } else {
            self.remove_overflow_flag();
        }
    }

    pub fn and_logical_and(&mut self, value: u8) {
        self.accumulator = self.accumulator & value;

        if self.accumulator == 0 {
            self.set_zero_flag();
        }

        if value > 127 {
            self.set_negative_flag()
        }
    }

    pub fn asl_arithmetic_shift_left(&mut self, ) {
        
    }

    pub fn lda_load_accumulator(&mut self, value: u8) {
        self.accumulator = value;

        if self.accumulator == 0 {
            self.status = self.status | 0b0000_0010;
        } else {
            self.status = self.status & 0b1111_1101;
        }

        if self.accumulator & 0b1000_0000 != 0  {
            self.status = self.status | 0b1000_0000;
        } else {
            self.status = self.status & 0b0111_1111;
        }
    }
}