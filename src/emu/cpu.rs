pub struct CPU {
    program_counter: u16,
    stack_pointer: u32,
    accumulator: u8,
    register_x: u32,
    register_y: u32,
    status: u8,
    memory: [u8; 0xFFFF]
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
            status: 0,
            memory: [0; 0xFFFF]
        }
    }

    fn read_mem(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn clc_clear_carry_flag(&mut self) {
        self.status = self.status & 0b1111_1110;
    }

    fn sec_set_carry_flag(&mut self) {
        self.status = self.status | 0b0000_0001;
    }

    fn set_zero_flag(&mut self) {
        self.status = self.status | 0b0000_0010;
    }

    fn set_overflow_flag(&mut self) {
        self.status = self.status | 0b0100_0000;
    }    

    fn remove_overflow_flag(&mut self) {
        self.status = self.status & 0b1011_1111;
    }   

    fn set_negative_flag(&mut self) {
        self.status = self.status | 0b1000_0000;
    }    

    fn remove_negative_flag(&mut self) {
        self.status = self.status & 0b0111_1111;
    }

    fn remove_zero_flag(&mut self) {
        self.status = self.status & 0b1111_1101;
    }

    fn get_carry_flag(&self) -> u8{
        return self.status & 0b0000_0001;
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

        self.accumulator = result;
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

    pub fn asl_arithmetic_shift_left(&mut self, value: u8) -> u8 {
        let result = value << 1;

        if value & 0b1000_0000 == 1 {
            self.sec_set_carry_flag();
        } else {
            self.clc_clear_carry_flag();
        }

        return result;
    }

    pub fn asl_arithmetic_shift_left_accu(&mut self) {
        let result = self.accumulator << 1;

        if self.accumulator & 0b1000_0000 == 1 {
            self.sec_set_carry_flag();
        } else {
            self.clc_clear_carry_flag();
        }

        if self.accumulator == 0 {
            self.set_zero_flag();
        } else {
            self.remove_zero_flag();
        }

        if result & 0b1000_0000 == 1 {
            self.set_negative_flag();
        } else {
            self.remove_negative_flag();
        }

        self.accumulator = result;
    }

    pub fn bcc_branch_if_carry_clear(&mut self) {
        if self.get_carry_flag() != 0 {
            return;
        }

        let addr = self.read_mem(self.program_counter) as i8;
        let jump_addr = self.
                        program_counter
                        .wrapping_add(1)
                        .wrapping_add(addr as u16);
        self.program_counter = jump_addr;

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