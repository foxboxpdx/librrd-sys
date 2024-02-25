use crate::{convert_to_mutmutchar, convert_to_mutmutmutchar, rrd_graph, RRAType, RRDCommand, __sFILE};
use std::collections::HashMap;
use std::ffi::c_int;
use std::ptr::null_mut;

// What should always get added to argv first
static ARGV0: &str = "graph";


#[derive(Debug, Default, Clone)]
pub struct Command {
    pub argc: i32,
    pub argv: Vec<String>,
    pub prdata: Vec<Vec<String>>, // ???
    pub xsize: i32,
    pub ysize: i32,
    pub ymin: i64,  // these want to be f64s but i can't
    pub ymax: i64,  // send them 'as *mut f64' so screw it
    pub filename: String,
    pub flags: Vec<String>, // For optionss with no args
    pub opts: HashMap<String, String>, // for options with required args
    pub defs: Vec<String>, // source definitions
    pub lines: Vec<String>, // line definitions
    pub gprints: Vec<String> // gprint definitions
}

pub struct Builder {
    pub data: Command
}

impl Builder {
    pub fn new(filename: String) -> Builder {
        let data = Command { filename, ..Default::default()};
        Builder { data }
    }
    
    pub fn build(&mut self) -> Command {
        let mut retval = self.data.clone();
        retval.argv.push(ARGV0.to_string());
        retval.argv.push(retval.filename.clone());
        for f in &self.data.flags {
            retval.argv.push(f.to_string());
        }
        for (k, v) in &self.data.opts {
            retval.argv.push(k.to_string());
            retval.argv.push(v.to_string());
        }
        retval.argv.append(&mut retval.defs);
        retval.argv.append(&mut retval.lines);
        retval.argv.append(&mut retval.gprints);
        retval.argc = retval.argv.len() as i32;
        //println!("argc: {}\nargv: {:?}", retval.argc, &retval.argv);
        retval
    }

    // flag
    pub fn alt_autoscale(mut self) -> Builder {
        self.data.flags.push("--alt-autoscale".to_string());
        self
    }

    // req
    pub fn imgformat(mut self, var: ImageFormat) -> Builder {
        self.data.opts.insert("--imgformat".to_string(), var.to_string());
        self
    }

    // req
    pub fn font_smoothing_threshold(mut self, var: &str) -> Builder {
        self.data.opts.insert("--font-smoothing-threshold".to_string(), var.to_string());
        self
    }

    //req
    pub fn base(mut self, var: &str) -> Builder {
        self.data.opts.insert("--base".to_string(), var.to_string());
        self
    }

    // req
    pub fn color(mut self, var: &str) -> Builder {
        self.data.opts.insert("--color".to_string(), var.to_string());
        self
    }

    // flag
    pub fn full_size_mode(mut self) -> Builder {
        self.data.flags.push("full-size-mode".to_string());
        self
    }

    //req
    pub fn daemon(mut self, var: &str) -> Builder {
        self.data.opts.insert("--daemon".to_string(), var.to_string());
        self
    }

    // flag
    pub fn slope_mode(mut self) -> Builder {
        self.data.flags.push("--slope-mode".to_string());
        self
    }

    // req
    pub fn end(mut self, var: &str) -> Builder {
        self.data.opts.insert("--end".to_string(), var.to_string());
        self
    }

    // flag
    pub fn force_rules_legend(mut self) -> Builder {
        self.data.flags.push("--force-rules-legend".to_string());
        self
    }

    // req
    pub fn imginfo(mut self, var: &str) -> Builder {
        self.data.opts.insert("--imginfo".to_string(), var.to_string());
        self
    }

    // req
    pub fn graph_render_mode(mut self, var: &str) -> Builder {
        self.data.opts.insert("--graph-render-mode".to_string(), var.to_string());
        self
    }

    // flag
    pub fn no_legend(mut self) -> Builder {
        self.data.flags.push("--no-legend".to_string());
        self
    }

    // req
    pub fn height(mut self, var: i32) -> Builder {
        self.data.opts.insert("--height".to_string(), var.to_string());
        self.data.ysize = var;
        self
    }

    // flag
    pub fn no_minor(mut self) -> Builder {
        self.data.flags.push("--no-minor".to_string());
        self
    }

    // flag
    pub fn interlaced(mut self) -> Builder {
        self.data.flags.push("--interlaced".to_string());
        self
    }

    // flag
    pub fn alt_autoscale_min(mut self) -> Builder {
        self.data.flags.push("--alt-autoscale-min".to_string());
        self
    }

    // flag
    pub fn only_graph(mut self) -> Builder {
        self.data.flags.push("--only-graph".to_string());
        self
    }

    // req
    pub fn units_length(mut self, var: &str) -> Builder {
        self.data.opts.insert("--units-length".to_string(), var.to_string());
        self
    }

    // req
    pub fn lower_limit(mut self, var: i64) -> Builder {
        self.data.opts.insert("--lower-limit".to_string(), var.to_string());
        self.data.ymin = var;
        self
    }

    // flag
    pub fn alt_autoscale_max(mut self) -> Builder {
        self.data.flags.push("--alt-autoscale-max".to_string());
        self
    }

    // req
    pub fn zoom(mut self, var: &str) -> Builder {
        self.data.opts.insert("--zoom".to_string(), var.to_string());
        self
    }

    // flag
    pub fn no_gridfit(mut self) -> Builder {
        self.data.flags.push("--no-gridfit".to_string());
        self
    }

    // req
    pub fn font(mut self, var: &str) -> Builder {
        self.data.opts.insert("--font".to_string(), var.to_string());
        self
    }

    // flag
    pub fn logarithmic(mut self) -> Builder {
        self.data.flags.push("--logarithmic".to_string());
        self
    }

    // flag
    pub fn pango_markup(mut self) -> Builder {
        self.data.flags.push("--pango-markup".to_string());
        self
    }

    // req
    pub fn font_render_mode(mut self, var: &str) -> Builder {
        self.data.opts.insert("--font-render-mode".to_string(), var.to_string());
        self
    }

    // flag
    pub fn rigid(mut self) -> Builder {
        self.data.flags.push("--rigid".to_string());
        self
    }
    
    // req
    pub fn step(mut self, var: &str) -> Builder {
        self.data.opts.insert("--step".to_string(), var.to_string());
        self
    }
    
    // req
    pub fn start(mut self, var: &str) -> Builder {
        self.data.opts.insert("--start".to_string(), var.to_string());
        self
    }
    
    // req
    pub fn tabwidth(mut self, var: &str) -> Builder {
        self.data.opts.insert("--tabwidth".to_string(), var.to_string());
        self
    }
    
    // req
    pub fn title(mut self, var: &str) -> Builder {
        self.data.opts.insert("--title".to_string(), var.to_string());
        self
    }
    
    // req
    pub fn upper_limit(mut self, var: i64) -> Builder {
        self.data.opts.insert("--upper-limit".to_string(), var.to_string());
        self.data.ymax = var;
        self
    }
    
    // req
    pub fn vertical_label(mut self, var: &str) -> Builder {
        self.data.opts.insert("--vertical-label".to_string(), var.to_string());
        self
    }
    
    // req
    pub fn watermark(mut self, var: &str) -> Builder {
        self.data.opts.insert("--watermark".to_string(), var.to_string());
        self
    }
    
    // req
    pub fn width(mut self, var: i32) -> Builder {
        self.data.opts.insert("--width".to_string(), var.to_string());
        self.data.xsize = var;
        self
    }
    
    // req
    pub fn units_exponent(mut self, var: &str) -> Builder {
        self.data.opts.insert("--units-exponent".to_string(), var.to_string());
        self
    }
    
    // req
    pub fn x_grid(mut self, var: &str) -> Builder {
        self.data.opts.insert("--x-grid".to_string(), var.to_string());
        self
    }
    
    // flag
    pub fn alt_y_grid(mut self) -> Builder {
        self.data.flags.push("--alt-y-grid".to_string());
        self
    }
    
    // req
    pub fn y_grid(mut self, var: &str) -> Builder {
        self.data.opts.insert("--y-grid".to_string(), var.to_string());
        self
    }
    
    // flag
    pub fn lazy(mut self) -> Builder {
        self.data.flags.push("--lazy".to_string());
        self
    }
    
    // flag
    pub fn use_nan_for_all_missing_data(mut self) -> Builder {
        self.data.flags.push("--use-nan-for-all-missing-data".to_string());
        self
    }
    
    // req
    pub fn units(mut self, var: &str) -> Builder {
        self.data.opts.insert("--units".to_string(), var.to_string());
        self
    }
    
    // flag
    pub fn add_jsontime(mut self) -> Builder {
        self.data.flags.push("--add-jsontime".to_string());
        self
    }
    
    // flag
    pub fn alt_y_mrtg(mut self) -> Builder {
        self.data.flags.push("--alt-y-mrtg".to_string());
        self
    }
    
    // flag
    pub fn disable_rrdtool_tag(mut self) -> Builder {
        self.data.flags.push("--disable-rrdtool-tag".to_string());
        self
    }
    
    // req
    pub fn right_axis(mut self, var: &str) -> Builder {
        self.data.opts.insert("--right-axis".to_string(), var.to_string());
        self
    }
    
    // req
    pub fn right_axis_label(mut self, var: &str) -> Builder {
        self.data.opts.insert("--right-axis-label".to_string(), var.to_string());
        self
    }
    
    // req
    pub fn right_axis_format(mut self, var: &str) -> Builder {
        self.data.opts.insert("--right-axis-format".to_string(), var.to_string());
        self
    }
    
    // req
    pub fn legend_position(mut self, var: &str) -> Builder {
        self.data.opts.insert("--legend-position".to_string(), var.to_string());
        self
    }
    
    // req
    pub fn legend_direction(mut self, var: &str) -> Builder {
        self.data.opts.insert("--legend-direction".to_string(), var.to_string());
        self
    }
    
    // req
    pub fn border(mut self, var: &str) -> Builder {
        self.data.opts.insert("--border".to_string(), var.to_string());
        self
    }
    
    // req
    pub fn grid_dash(mut self, var: &str) -> Builder {
        self.data.opts.insert("--grid-dash".to_string(), var.to_string());
        self
    }
    
    // flag
    pub fn dynamic_labels(mut self) -> Builder {
        self.data.flags.push("--dynamic-labels".to_string());
        self
    }
    
    // req
    pub fn week_fmt(mut self, var: &str) -> Builder {
        self.data.opts.insert("--week-fmt".to_string(), var.to_string());
        self
    }
    
    // req
    pub fn graph_type(mut self, var: &str) -> Builder {
        self.data.opts.insert("--graph-type".to_string(), var.to_string());
        self
    }
    
    // req
    pub fn left_axis_format(mut self, var: &str) -> Builder {
        self.data.opts.insert("--left-axis-format".to_string(), var.to_string());
        self
    }
    
    // req
    pub fn left_axis_formatter(mut self, var: &str) -> Builder {
        self.data.opts.insert("--left-axis-formatter".to_string(), var.to_string());
        self
    }
    
    // req
    pub fn right_axis_formatter(mut self, var: &str) -> Builder {
        self.data.opts.insert("--right-axis-formatter".to_string(), var.to_string());
        self
    }
    
    // flag
    pub fn allow_shrink(mut self) -> Builder {
        self.data.flags.push("--allow-shrink".to_string());
        self
    }
    
    // flag
    pub fn utc(mut self) -> Builder {
        self.data.flags.push("--utc".to_string());
        self
    }

    // Add a DEF
    pub fn with_def(mut self, name: &str, rrd: &str, source: &str, rra: RRAType) -> Builder {
        let s = format!("DEF:{}={}:{}:{}", name, rrd, source, rra.to_string());
        self.data.defs.push(s);
        self
    }

    // Add a LINE
    // Slightly trickier since we need to keep track of the count
    pub fn with_line(mut self, source: &str, color: &str, desc: &str) -> Builder {
        let line_num = self.data.lines.len() + 1;
        let s = format!("LINE{}:{}{}:{}", line_num, source, color, desc);
        self.data.lines.push(s);
        self
    }

    // Add a GPRINT
    pub fn with_gprint(mut self, source: &str, rra: RRAType, desc: &str) -> Builder {
        let s = format!("GPRINT:{}:{}:\"{}\"", source, rra.to_string(), desc);
        self.data.gprints.push(s);
        self
    }

}

// rrd_graph:
// i32, mut mut i8, mut mut mut i8, i32, i32, mut __sFILE, f64, f64 -> i32
// argc,   argv   ,    prdata     , xsize, ysize, stream , ymin, ymax
// 
// In the python bindings, 'prdata' is "char **calcpr = NULL"
// And 'stream' is simply NULL
impl RRDCommand for Command {
    fn execute(&self) -> bool {
        unsafe {
            let mut converted = convert_to_mutmutchar(self.argv.clone());
            let empty: Vec<Vec<String>> = Vec::new();
            let mut conv2 = convert_to_mutmutmutchar(empty);
            let mut uninit: ::std::mem::MaybeUninit<__sFILE> = ::std::mem::MaybeUninit::uninit();
            //let prdata: *mut *mut *mut i8 = null_mut();
            //let stream: *mut __sFILE = null_mut();
            rrd_graph(self.argc as c_int, 
                converted.as_mut_ptr(), 
                conv2.as_mut_ptr(), 
                self.xsize as *mut i32, 
                self.ysize as *mut i32, 
                uninit.as_mut_ptr(),
                self.ymin as *mut f64,
                self.ymax as *mut f64) == 0
        }
    }
}

pub enum ImageFormat {
    PNG,SVG,EPS,PDF,XML,XMLENUM,JSON,JSONTIME,CSV,TSV,SSV
}
impl std::fmt::Display for ImageFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ImageFormat::CSV       => write!(f, "CSV"),
            ImageFormat::EPS       => write!(f, "EPS"),
            ImageFormat::JSON      => write!(f, "JSON"),
            ImageFormat::JSONTIME  => write!(f, "JSONTIME"),
            ImageFormat::PDF       => write!(f, "PDF"),
            ImageFormat::PNG       => write!(f, "PNG"),
            ImageFormat::SSV       => write!(f, "SSV"),
            ImageFormat::SVG       => write!(f, "SVG"),
            ImageFormat::TSV       => write!(f, "TSV"),
            ImageFormat::XML       => write!(f, "XML"),
            ImageFormat::XMLENUM   => write!(f, "XMLENUM")
        }
    }
}