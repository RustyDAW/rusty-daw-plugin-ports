pub const TABLE_SIZE: usize = 4001;

#[allow(non_camel_case_types)]
#[allow(dead_code)]
pub enum Table {
    Sqrt,
    Sqrt3,
    Pow1_5,
    Pow2,
    Pow3,
    Pow4,
    X2Pow3,
    Response2_Oct,
    Response3_Oct,
    Response4_Oct,
    Response5_Oct,
    Response6_Oct,
    Response2_Dec,
    Response3_Dec,
    Response4_Dec,
}

pub struct ValueTables {
    pub sqrt: [f64; TABLE_SIZE],
    pub sqrt3: [f64; TABLE_SIZE],
    pub pow1_5: [f64; TABLE_SIZE],
    pub pow2: [f64; TABLE_SIZE],
    pub pow3: [f64; TABLE_SIZE],
    pub pow4: [f64; TABLE_SIZE],
    pub x2pow3: [f64; TABLE_SIZE],
    pub response2_oct: [f64; TABLE_SIZE], // "octave", not "octal"; value 2x every step
    pub response3_oct: [f64; TABLE_SIZE],
    pub response4_oct: [f64; TABLE_SIZE],
    pub response5_oct: [f64; TABLE_SIZE],
    pub response6_oct: [f64; TABLE_SIZE],
    pub response2_dec: [f64; TABLE_SIZE], // "decade", not "decimal"; value 10x every step
    pub response3_dec: [f64; TABLE_SIZE],
    pub response4_dec: [f64; TABLE_SIZE],
}

impl ValueTables {
    pub fn new() -> Self {
        let mut value_tables = Self {
            sqrt: [0.0; TABLE_SIZE],
            sqrt3: [0.0; TABLE_SIZE],
            pow1_5: [0.0; TABLE_SIZE],
            pow2: [0.0; TABLE_SIZE],
            pow3: [0.0; TABLE_SIZE],
            pow4: [0.0; TABLE_SIZE],
            x2pow3: [0.0; TABLE_SIZE],
            response2_oct: [0.0; TABLE_SIZE],
            response3_oct: [0.0; TABLE_SIZE],
            response4_oct: [0.0; TABLE_SIZE],
            response5_oct: [0.0; TABLE_SIZE],
            response6_oct: [0.0; TABLE_SIZE],
            response2_dec: [0.0; TABLE_SIZE],
            response3_dec: [0.0; TABLE_SIZE],
            response4_dec: [0.0; TABLE_SIZE],
        };

        for i in 0..TABLE_SIZE {
            let x = (i as f64) / 4000.0;
            value_tables.sqrt[i] = x.sqrt();
            value_tables.sqrt3[i] = x.powf(1.0/3.0);
            value_tables.pow1_5[i] = x.powf(1.5);
            value_tables.pow2[i] = x.powf(2.0);
            value_tables.pow3[i] = x.powf(3.0);
            value_tables.pow4[i] = x.powf(4.0);
            value_tables.x2pow3[i] = (2.0 * x).powf(3.0);
            value_tables.response2_oct[i] = ((4.0f64).powf(x) - 1.0) / 4.0 + (1.0 / 4.0);
            value_tables.response3_oct[i] = ((8.0f64).powf(x) - 1.0) / 8.0 + (1.0 / 8.0);
            value_tables.response4_oct[i] = ((16.0f64).powf(x) - 1.0) / 16.0 + (1.0 / 16.0);
            value_tables.response5_oct[i] = ((32.0f64).powf(x) - 1.0) / 32.0 + (1.0 / 32.0);
            value_tables.response6_oct[i] = ((64.0f64).powf(x) - 1.0) / 64.0 + (1.0 / 64.0);
            value_tables.response2_dec[i] = (100.0f64).powf(x) / 100.0;
            value_tables.response3_dec[i] = (1000.0f64).powf(x) / 1000.0;
            value_tables.response4_dec[i] = (10000.0f64).powf(x) / 10000.0;
        }

        for i in 0..TABLE_SIZE {
            value_tables.response2_oct[i] = (value_tables.response2_oct[i] - value_tables.response2_oct[0]) / (1.0 - value_tables.response2_oct[0]);
            value_tables.response3_oct[i] = (value_tables.response3_oct[i] - value_tables.response3_oct[0]) / (1.0 - value_tables.response3_oct[0]);
            value_tables.response4_oct[i] = (value_tables.response4_oct[i] - value_tables.response4_oct[0]) / (1.0 - value_tables.response4_oct[0]);
            value_tables.response5_oct[i] = (value_tables.response5_oct[i] - value_tables.response5_oct[0]) / (1.0 - value_tables.response5_oct[0]);
            value_tables.response6_oct[i] = (value_tables.response6_oct[i] - value_tables.response6_oct[0]) / (1.0 - value_tables.response6_oct[0]);
            value_tables.response2_dec[i] = (value_tables.response2_dec[i] - value_tables.response2_dec[0]) / (1.0 - value_tables.response2_dec[0]);
            value_tables.response3_dec[i] = (value_tables.response3_dec[i] - value_tables.response3_dec[0]) / (1.0 - value_tables.response3_dec[0]);
            value_tables.response4_dec[i] = (value_tables.response4_dec[i] - value_tables.response4_dec[0]) / (1.0 - value_tables.response4_dec[0]);
        }

        value_tables.response2_oct[0] = 0.0;
        value_tables.response3_oct[0] = 0.0;
        value_tables.response4_oct[0] = 0.0;
        value_tables.response5_oct[0] = 0.0;
        value_tables.response6_oct[0] = 0.0;
        value_tables.response2_dec[0] = 0.0;
        value_tables.response3_dec[0] = 0.0;
        value_tables.response4_dec[0] = 0.0;

        value_tables
    }

    pub fn get(&self, table_name: Table, index: f64) -> f64 {
        let idx = (index * 4000.999) as usize;
        assert!(idx < TABLE_SIZE);
        match table_name {
            Table::Sqrt => self.sqrt[idx],
            Table::Sqrt3 => self.sqrt3[idx],
            Table::Pow1_5 => self.pow1_5[idx],
            Table::Pow2 => self.pow2[idx],
            Table::Pow3 => self.pow3[idx],
            Table::Pow4 => self.pow4[idx],
            Table::X2Pow3 => self.x2pow3[idx],
            Table::Response2_Oct => self.response2_oct[idx],
            Table::Response3_Oct => self.response3_oct[idx],
            Table::Response4_Oct => self.response4_oct[idx],
            Table::Response5_Oct => self.response5_oct[idx],
            Table::Response6_Oct => self.response6_oct[idx],
            Table::Response2_Dec => self.response2_dec[idx],
            Table::Response3_Dec => self.response3_dec[idx],
            Table::Response4_Dec => self.response4_dec[idx],
        }
    }
}