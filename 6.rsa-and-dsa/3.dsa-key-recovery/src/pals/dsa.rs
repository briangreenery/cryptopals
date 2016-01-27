use super::BigNum;
use super::sha1;

pub struct DSA {
    p: BigNum,
    q: BigNum,
    g: BigNum,
    x: BigNum,
    y: BigNum,
}

impl DSA {
    pub fn new(p: BigNum, q: BigNum, g: BigNum) -> DSA {
        let x = BigNum::random_less_than(&q);
        let y = g.modexp(&x, &p);

        DSA {
            p: p,
            q: q,
            g: g,
            x: x,
            y: y,
        }
    }

    pub fn sign(&self, bytes: &[u8]) -> (BigNum, BigNum) {
        let zero = BigNum::new(0);
        let hash = BigNum::from_bytes(&sha1::hash(&bytes));

        loop {
            let k = BigNum::random_less_than(&self.q);
            if k == zero {
                continue;
            }

            let r = self.g.modexp(&k, &self.p).div(&self.q).1;
            if r == zero {
                continue;
            }

            let xr = self.x.mul(&r);
            let kinv = k.modinv(&self.q);

            let s = kinv.mul(&hash.add(&xr)).div(&self.q).1;
            if s == zero {
                continue;
            }

            return (r, s);
        }
    }

    pub fn verify(&self, bytes: &[u8], signature: &(BigNum, BigNum)) -> bool {
        let r = &signature.0;
        let s = &signature.1;

        let zero = BigNum::new(0);
        let hash = BigNum::from_bytes(&sha1::hash(&bytes));

        if r <= &zero || r >= &self.q {
            return false;
        }

        if s <= &zero || s >= &self.q {
            return false;
        }

        let w = s.modinv(&self.q);
        let u1 = hash.mul(&w).div(&self.q).1;
        let u2 = r.mul(&w).div(&self.q).1;

        let g_u1 = self.g.modexp(&u1, &self.p);
        let y_u2 = self.y.modexp(&u2, &self.p);

        let v = g_u1.mul(&y_u2).div(&self.p).1.div(&self.q).1;

        r == &v
    }
}
