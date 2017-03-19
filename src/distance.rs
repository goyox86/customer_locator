pub enum Distance {
    Mm(f64),
    Cm(f64),
    Mt(f64),
    Km(f64),
}

impl Distance {
    pub fn to_mm(&self) -> Distance {
        match *self {
            Distance::Mm(_) => *self,
            Distance::Cm(cms) => Distance::Cm(cms / 100.0),
            Distance::Mt(mts) => Distance::Mt(mts / 1000.0),
            Distance::Km(kms) => Distance::Km(kms / 1000000.0),         
        }               
    }

    pub fn to_cm(&self) -> Distance {
        match *self {
            Distance::Mm(mms) => Distance::Mm(mms * 100.0),
            Distance::Cm(_) => *self,
            Distance::Mt(mts) => Distance::Mt(mts / 100.0),
            Distance::Km(kms) => Distance::Km(kms / 1000.0),         
        }    
    }

    pub fn to_mt(&self) -> Distance {
        match *self {
            Distance::Mm(mms) => Distance::Mm(mms / 1000.0),
            Distance::Cm(cms) => Distance::Cm(cms / 100.0),
            Distance::Mt(_) => *self,
            Distance::Km(kms) => Distance::Km(kms / 1000.0),         
        } 
    }

    pub fn to_km(&self) -> Distance {
        match *self {
            Distance::Mm(mms) => Distance::Mm(mms / 1000000.0),
            Distance::Cm(cms) => Distance::Cm(cms / 100000.0),
            Distance::Mt(mts) => Distance::Mt(mts / 1000.0),
            Distance::Km(kms) => *self,         
        }         
    }
}

impl fmt::Display for Distance {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Distance::Mm(mms) => write!(f, "{} milimieters", mms),
            Distance::Cm(cms) => write!(f, "{} centimeters", cms),
            Distance::Mt(mts) => write!(f, "{} meters", mts),
            Distance::Km(kms) => write!(f, "{} kilometers", kms),
        }
    }
}
