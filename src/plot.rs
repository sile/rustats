use crate::distributions::Pdf;
use crate::range::Range;
use crate::{Error, Result};
use std::fs::File;
use std::io::Write;
use std::path::Path;

// ```gnuplot
// set contour
// splot "FILE" w l
// ```
#[derive(Debug)]
pub struct Density2d<'a, D> {
    distribution: &'a D,
    pub xrange: Range<f64>,
    pub yrange: Range<f64>,
    pub xsamples: usize,
    pub ysamples: usize,
}
impl<'a, D> Density2d<'a, D>
where
    D: Pdf<(f64, f64)>,
{
    pub fn new(distribution: &'a D) -> Self {
        Self {
            distribution,
            xrange: Range {
                low: 0.0,
                high: 1.0,
            },
            yrange: Range {
                low: 0.0,
                high: 1.0,
            },
            xsamples: 100,
            ysamples: 100,
        }
    }

    pub fn to_writer<W: Write>(&self, mut writer: W) -> Result<()> {
        track!(writeln!(writer, "# x y z(density)").map_err(Error::from))?;
        for x in self.xrange.iter(self.xrange.width() / self.xsamples as f64) {
            for y in self.yrange.iter(self.yrange.width() / self.ysamples as f64) {
                let density = self.distribution.pdf(&(x, y));
                track!(writeln!(writer, "{} {} {}", x, y, density).map_err(Error::from))?;
            }
            track!(writeln!(writer).map_err(Error::from))?;
        }
        track!(writer.flush().map_err(Error::from))?;
        Ok(())
    }

    pub fn to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let f = track!(File::create(&path).map_err(Error::from); path.as_ref())?;
        track!(self.to_writer(f))
    }
}
