use fountain_engine::DataManager;
use fountain_engine::algebra::finite_field::GF256;
// use fountain_engine::algebra::finite_field::{GF2, GF256};
// use fountain_engine::algebra::linear_algebra::Matrix;
use fountain_engine::traits::{LDPC, HDPC};
use fountain_engine::types::{CodeParams};
use fountain_scheme::precodes::hdpc_binary::R10HDPC;
use std::sync::{Arc, OnceLock};


pub struct CachedR10HDPC {
    inner: R10HDPC,
    cached_lu: Arc<OnceLock<(CodeParams, Vec<usize>, Vec<Vec<u8>>)>>,
}

impl CachedR10HDPC {
    /// Creates a new Raptor-10 binary HDPC precode instance.
    #[must_use]
    pub fn new(cache: Arc<OnceLock<(CodeParams, Vec<usize>, Vec<Vec<u8>>)>>) -> Self {
        Self {
            inner: R10HDPC::new(),
            cached_lu: cache,
        }
    }
}


impl HDPC for CachedR10HDPC {
    fn mul_data(
        &self,
        manager: &mut DataManager,
        params: &CodeParams,
        x_ids: &[usize],
        y_ids: &[usize],
    ) {
        self.inner.mul_data(manager, params, x_ids, y_ids)
    }

    fn mul_binary(
        &self,
        _gf: Option<&GF256>,
        params: &CodeParams,
        n: usize,
        v: &dyn Fn(usize) -> Vec<u8>,
    ) -> Vec<Vec<u8>> {
        self.inner.mul_binary(_gf, params, n, v)
    }

    fn mul_sparse(
        &self,
        _gf: Option<&GF256>,
        params: &CodeParams,
        n: usize,
        s: &dyn Fn(usize) -> Vec<usize>,
    ) -> Vec<Vec<u8>> {
        self.inner.mul_sparse(_gf, params, n, s)
    }

    fn mul_sparse_sh(
        &self,
        _gf: Option<&GF256>,
        params: &CodeParams,
        s: &dyn Fn(usize) -> Vec<usize>,
    ) -> Vec<Vec<u8>> {
        self.inner.mul_sparse_sh(_gf, params, s)
    }

    /// [`GF2_FIELD_POLY`]: binary HDPC, not GF(256).
    fn gf_poly(&self) -> u16 {
        self.inner.gf_poly()
    }

    fn lu_idssh(
        &self,
        gf: Option<&GF256>,
        params: &CodeParams,
        ldpc: &dyn LDPC,
    ) -> (Vec<usize>, Vec<Vec<u8>>) {
        if let Some(cached) = self.cached_lu.get() {
            if cached.0.k == params.k
                && cached.0.a == params.a
                && cached.0.l == params.l
                && cached.0.h == params.h {
                return (cached.1.clone(), cached.2.clone());
            }
        }

        let (p, m) = self.inner.lu_idssh(gf, params, ldpc);
        let _ = self.cached_lu.set((params.clone(), p.clone(), m.clone()));

        // let sh_column = |row: usize| {
        //     ldpc.inactive_row(row)
        //         .into_iter()
        //         .filter(|&id| id >= params.b)
        //         .map(|id| id - params.b)
        //         .collect::<Vec<_>>()
        // };

        // let mut m = self.mul_sparse_sh(gf, params, &sh_column);
        // for (i, row) in m.iter_mut().enumerate().take(params.h) {
        //     row[i] ^= 1; // GF(256) addition is XOR in this codebase
        // }

        // let (p, r) = match gf {
        //     Some(gf) => Matrix::lu_decomp(gf, &mut m),
        //     None => Matrix::lu_decomp(&GF2::new(), &mut m),
        // }; 

        // *self.cached_lu.borrow_mut() = Some((p.clone(), m.clone()));
        // assert_eq!(r, params.h, "I' + D_s S_h singular, rank {r} (dense fallback also failed)");

        (p, m)
    }
}





