use std::cell::RefCell;
use std::iter::Peekable;
use std::rc::Rc;

// a file is a Tree with no children;
// a directory is a Tree with no size
struct Tree {
    name: String,
    filesize: i64,
    children: Vec<Rc<RefCell<Tree>>>,
    parent: Option<Rc<RefCell<Tree>>>,
}

fn read_directory(current_working_directory: Rc<RefCell<Tree>>, line_iter: &mut Peekable<std::str::Lines>) {
    loop {
        match line_iter.peek() {
            Some(text) => {
                if text.chars().nth(0)==Some('$') {
                    break;
                }
            }
            None => {
                break;
            }
        }
        let line = line_iter.next().unwrap();
        let mut split = line.split(' ');
        match split.next() {
            Some(text) => {
                match text {
                    "$" => {
                        break;
                    }
                    "dir" => {
                        let new_folder = Rc::new(RefCell::new(Tree {
                            name: split.next().unwrap().to_string(),
                            filesize: 0,
                            parent: Some(Rc::clone(&current_working_directory)),
                            children: Vec::new(),
                        }));
                        current_working_directory.borrow_mut().children.push(new_folder);
                    }
                    _ => {
                        let new_file = Rc::new(RefCell::new(Tree {
                            name: split.next().unwrap().to_string(),
                            filesize: text.parse::<i64>().unwrap(),
                            parent: Some(Rc::clone(&current_working_directory)),
                            children: Vec::new(),
                        }));
                        current_working_directory.borrow_mut().children.push(new_file);
                    }
                }
            }
            None => {
                break;
            }
        }
    }
}

fn deduce_filesystem_from_input(input: String) -> Rc<RefCell<Tree>> {
    let root = Rc::new(RefCell::new(Tree {
        name: "".to_string(),
        filesize: 0,
        parent: None,
        children: Vec::new(),
    }));
    let mut current_working_directory = Rc::clone(&root);
    let mut line_iter: Peekable<std::str::Lines> = input.lines().into_iter().peekable();
    loop {
        match line_iter.next() {
            Some(line) => {
                let mut split = line.split(' ');
                debug_assert!(split.next()==Some("$"));
                match split.next() {
                    Some(text) => {
                        match text {
                            "ls" => {
                                read_directory(current_working_directory.clone(), &mut line_iter);
                            }
                            "cd" => {
                                let target_directory = split.next().unwrap();
                                match target_directory {
                                    "/" => {
                                        current_working_directory = Rc::clone(&root);
                                    }
                                    ".." => { 
                                        let new_working_directory = match &current_working_directory.borrow().parent {
                                            Some(directory) => {
                                                Rc::clone(&directory)
                                            }
                                            None => { 
                                                assert!(false);
                                                Rc::clone(&current_working_directory)
                                            }
                                        };
                                        current_working_directory = new_working_directory;
                                    }
                                    _ => {
                                        let new_working_directory = match current_working_directory.borrow().children.iter().find(|child| child.borrow().name==target_directory) {
                                            Some(found_directory) => {
                                                Rc::clone(found_directory)
                                            }
                                            None => {
                                                assert!(false);
                                                Rc::clone(&current_working_directory)
                                            }
                                        };
                                        current_working_directory = new_working_directory;
                                    }
                                }
                            }
                            _ => {
                                println!("Unknown command");
                                break;
                            }
                        }
                    }
                    None => {
                        println!("{line} ended prematurely");
                    }
                }
            }
            None => {
                println!("End of lines");
                break;
            }
        }
    }
    return root;
}

fn directory_size(directory: &Rc<RefCell<Tree>>) -> i64 {
    directory.borrow().filesize
        + directory.borrow().children.iter().map(|child| directory_size(child)).sum::<i64>()
}

// leaves out files, only keeps folders
fn flatten_tree(directory: &Rc<RefCell<Tree>>) -> Vec<Rc<RefCell<Tree>>> {
    if directory.borrow().filesize > 0 {
        vec![]
    }
    else {
        let borrowed_directory = directory.borrow();
        println!("unroll_tree {}", borrowed_directory.name);
        let children_iter = borrowed_directory.children.iter();
        let child_lists = children_iter.map(|child| flatten_tree(child));
        let mut flattened = vec![Rc::clone(directory)];
        for list in child_lists {
            flattened.extend(list);
        }
        
        flattened
    }
}

fn sum_of_sizes_under_limit(root: &Rc<RefCell<Tree>>) -> i64 {
    let directories = flatten_tree(root);
    let mut sum = 0;
    for directory in directories {
        let my_directory_size = directory_size(&directory);
        if my_directory_size <= 100000 {
            sum += my_directory_size;
        }
    }
    
    sum
}

fn size_of_best_directory_to_delete(root: &Rc<RefCell<Tree>>) -> Option<i64> {
    let total_size = 70000000;
    let free_space_needed = 30000000;
    let current_space_used = directory_size(&root);
    let current_unused_space = total_size - current_space_used;
    let space_needed = free_space_needed - current_unused_space;
    let directories = flatten_tree(root);
    let directory_sizes = 
        directories.iter().map(|dir| {directory_size(&dir)})
                          .filter(|x| {x>=&space_needed});
    
    directory_sizes.min()
}

#[test]
fn read_directory_sample() {
    let root = Rc::new(RefCell::new(Tree {
        name: "".to_string(),
        filesize: 0,
        children: Vec::new(),
        parent: None,
    }));
    let sample_input = "dir a\n14848514 b.txt\n8504156 c.dat\ndir d";
    let mut line_iter = sample_input.lines().peekable();
    read_directory(root.clone(), &mut line_iter);
    assert_eq!("a", root.borrow().children[0].borrow().name);
    assert_eq!(0, root.borrow().children[0].borrow().filesize);
    assert_eq!("b.txt", root.borrow().children[1].borrow().name);
    assert_eq!(14848514, root.borrow().children[1].borrow().filesize);
    assert_eq!("c.dat", root.borrow().children[2].borrow().name);
    assert_eq!(8504156, root.borrow().children[2].borrow().filesize);
    assert_eq!("d", root.borrow().children[3].borrow().name);
    assert_eq!(14848514+8504156, directory_size(&root));
}

#[test]
fn empty_directory_0_size() {
    let root = Rc::new(RefCell::new(Tree {
        name: "".to_string(),
        filesize: 0,
        children: Vec::new(),
        parent: None,
    }));
    assert_eq!(0, directory_size(&root));
}

#[test]
fn unroll_tree_test() {
    let root = Rc::new(RefCell::new(Tree {
        name: "".to_string(),
        filesize: 0,
        children: Vec::new(),
        parent: None,
    }));
    let sample_input = "dir a\n14848514 b.txt\n8504156 c.dat\ndir d";
    let mut line_iter = sample_input.lines().peekable();
    read_directory(root.clone(), &mut line_iter);
    let result = flatten_tree(&root);
    assert_eq!(3, result.len());
    assert_eq!("", result[0].borrow().name);
    assert_eq!("a", result[1].borrow().name);
    assert_eq!("d", result[2].borrow().name);
}

#[test]
fn sample_integration() {
   let root = deduce_filesystem_from_input(
"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k".to_string());
    assert_eq!(4, root.borrow().children.len());

    assert_eq!("a", root.borrow().children[0].borrow().name);
    assert_eq!(94853, directory_size(&root.borrow().children[0])); // directory 'a'
    assert_eq!(0, root.borrow().children[0].borrow().filesize);
    
    assert_eq!("b.txt", root.borrow().children[1].borrow().name);
    assert_eq!(14848514, root.borrow().children[1].borrow().filesize);
    assert_eq!("c.dat", root.borrow().children[2].borrow().name);
    assert_eq!(8504156, root.borrow().children[2].borrow().filesize);
    
    assert_eq!("d", root.borrow().children[3].borrow().name);
    assert_eq!(24933642, directory_size(&root.borrow().children[3])); // directory 'd'

    assert_eq!("e", root.borrow().children[0].borrow().children[0].borrow().name);
    assert_eq!(584, directory_size(&root.borrow().children[0].borrow().children[0])); // directory 'e'

    assert_eq!("j", root.borrow().children[3].borrow().children[0].borrow().name);
    assert_eq!(48381165, directory_size(&root));

    assert_eq!(95437, sum_of_sizes_under_limit(&root));

    assert_eq!(Some(24933642), size_of_best_directory_to_delete(&root));
}

fn main() {
    let puzzle_input = 
"$ cd /
$ ls
dir bnl
dir dmpsnhdh
272080 dncdssn.hdr
dir fcnqg
6067 hjpmqrq
dir jvwtm
dir ldztz
dir lmmw
dir wthvqw
dir zpdnprb
$ cd bnl
$ ls
dir dhw
dir dmpsnhdh
dir lmw
dir vgbqbrst
$ cd dhw
$ ls
237421 vccwmhl
$ cd ..
$ cd dmpsnhdh
$ ls
dir chf
dir mjpbhjm
dir zwhpwp
$ cd chf
$ ls
4679 lmw.wmp
217367 wwnfv.qqr
dir zfgznbz
$ cd zfgznbz
$ ls
179409 cnj.gdn
171574 vglqg
$ cd ..
$ cd ..
$ cd mjpbhjm
$ ls
dir crf
dir hqnj
dir lmw
18783 lmw.rwr
302608 twpq
166891 vqczlg
$ cd crf
$ ls
32183 dltmqht
240428 frqqdsr.hbf
224910 sgtnrvrt
$ cd ..
$ cd hqnj
$ ls
261723 cgstb
77979 dmpsnhdh.cmd
$ cd ..
$ cd lmw
$ ls
50307 fcqrwd
$ cd ..
$ cd ..
$ cd zwhpwp
$ ls
141133 gdngm.mps
$ cd ..
$ cd ..
$ cd lmw
$ ls
dir dvv
267473 jmqgrh.dlz
295139 rrqjwpm
$ cd dvv
$ ls
114536 gmlmbrrw.wdm
102061 lmw
$ cd ..
$ cd ..
$ cd vgbqbrst
$ ls
105102 dmpsnhdh.bgl
269054 gmwgjf.fzz
dir jbdtpnw
245266 jzsjvgl
216220 lmw.gtb
dir rflp
dir twpq
$ cd jbdtpnw
$ ls
27543 cjvvmzp
$ cd ..
$ cd rflp
$ ls
137601 frqqdsr.hbf
83444 rrqjwpm
$ cd ..
$ cd twpq
$ ls
dir rlbsdj
36846 tnrqzjdd
$ cd rlbsdj
$ ls
56078 bvndq
$ cd ..
$ cd ..
$ cd ..
$ cd ..
$ cd dmpsnhdh
$ ls
dir fnpwwhtj
dir lmw
9090 mgjpsvl.jlh
186374 pbb.zln
$ cd fnpwwhtj
$ ls
dir cgp
$ cd cgp
$ ls
81938 hjpmqrq
281971 jvszf
151057 wmr.bnf
$ cd ..
$ cd ..
$ cd lmw
$ ls
dir bfbv
56929 pbb.zln
dir rrqjwpm
dir sngm
$ cd bfbv
$ ls
92667 qrrttb.jgp
$ cd ..
$ cd rrqjwpm
$ ls
25739 cqljn.zqw
91325 dncdssn.hdr
$ cd ..
$ cd sngm
$ ls
282163 jgrj
dir lmw
237524 lmw.dff
153497 lmw.ntg
dir lqd
dir szn
143535 tvpvc.qpr
98326 vbfgh
$ cd lmw
$ ls
32484 dncdssn.hdr
dir glwr
$ cd glwr
$ ls
144719 frqqdsr.hbf
$ cd ..
$ cd ..
$ cd lqd
$ ls
231401 dncdssn.hdr
dir jnjqmvg
dir lmw
199704 rrqjwpm
$ cd jnjqmvg
$ ls
104947 trpsrfjz.brg
$ cd ..
$ cd lmw
$ ls
230298 rrqjwpm.nnv
158947 wfv.qrb
$ cd ..
$ cd ..
$ cd szn
$ ls
197974 frqqdsr.hbf
$ cd ..
$ cd ..
$ cd ..
$ cd ..
$ cd fcnqg
$ ls
251609 dncdssn.hdr
289497 jdjmftqs
228459 qbmthcq
$ cd ..
$ cd jvwtm
$ ls
dir dmpsnhdh
47959 pbb.zln
dir tlr
dir twpq
dir wbgcsw
dir zjmldjdh
$ cd dmpsnhdh
$ ls
247567 bnl
102471 bnl.wdm
80054 fhqvp.hfm
dir llhp
dir mnsbh
dir mpplsfjp
20844 mtvl.lmp
$ cd llhp
$ ls
180255 dmpsnhdh
$ cd ..
$ cd mnsbh
$ ls
267627 dmpsnhdh
$ cd ..
$ cd mpplsfjp
$ ls
dir bnl
233742 tcnpvqc.tdr
$ cd bnl
$ ls
243223 fcqrwd
$ cd ..
$ cd ..
$ cd ..
$ cd tlr
$ ls
dir vcsngm
dir wndmt
$ cd vcsngm
$ ls
36434 czs.dnv
$ cd ..
$ cd wndmt
$ ls
dir fvmtfcqd
dir nvdb
dir nwqqgl
dir sbspgnpm
$ cd fvmtfcqd
$ ls
237025 nzttjt.rzh
$ cd ..
$ cd nvdb
$ ls
235328 dnrqwqtp.vfc
51984 rhblt.mfz
51332 rjhvhw
$ cd ..
$ cd nwqqgl
$ ls
203534 cjghw
dir cljbrh
$ cd cljbrh
$ ls
133820 lmw.dnd
$ cd ..
$ cd ..
$ cd sbspgnpm
$ ls
270010 hjpmqrq
$ cd ..
$ cd ..
$ cd ..
$ cd twpq
$ ls
dir dmpsnhdh
dir hbchdjjp
247649 hjpmqrq
29891 rrqjwpm
72407 twpq.crb
$ cd dmpsnhdh
$ ls
251490 dncdssn.hdr
222231 hjpmqrq
102058 pbb.zln
$ cd ..
$ cd hbchdjjp
$ ls
70058 dprrmd.qcd
230958 tsdbl.bnq
$ cd ..
$ cd ..
$ cd wbgcsw
$ ls
292028 bhtfcf
dir bnl
dir bqq
dir ctnlpgt
247888 hblhfvwj
dir hbqm
277949 pbb.zln
106225 rrqjwpm
125927 ssqpmlfb.gwm
dir zqmjwsgz
dir zwwphs
$ cd bnl
$ ls
dir fbbr
240500 frqqdsr.hbf
dir mzfrdl
25137 srqlww.mcj
dir tqgrdz
dir ztrnq
$ cd fbbr
$ ls
84414 mjbw.dhs
$ cd ..
$ cd mzfrdl
$ ls
135647 bccwgn
dir cjdptqgh
dir hwdnrqns
dir prq
$ cd cjdptqgh
$ ls
147946 mdgl.drz
11972 pbb.zln
$ cd ..
$ cd hwdnrqns
$ ls
dir dmpsnhdh
$ cd dmpsnhdh
$ ls
254432 phthmn
$ cd ..
$ cd ..
$ cd prq
$ ls
75827 dmpsnhdh.rtl
$ cd ..
$ cd ..
$ cd tqgrdz
$ ls
251475 bjwnll.rlw
$ cd ..
$ cd ztrnq
$ ls
208497 bnl.dtr
179376 gqnbswcj.hht
$ cd ..
$ cd ..
$ cd bqq
$ ls
202201 bnl.lbm
$ cd ..
$ cd ctnlpgt
$ ls
269484 vsfvzrpr
$ cd ..
$ cd hbqm
$ ls
74455 bvnfz
42748 pbb.zln
$ cd ..
$ cd zqmjwsgz
$ ls
146194 pbb.zln
$ cd ..
$ cd zwwphs
$ ls
209587 mtbzd.nwb
$ cd ..
$ cd ..
$ cd zjmldjdh
$ ls
dir cdq
dir mdclfbs
dir tfc
132043 wrm
$ cd cdq
$ ls
289173 twpq.mrn
$ cd ..
$ cd mdclfbs
$ ls
64639 bnl.jwf
dir hpdgt
72868 hznfj.nmj
159467 lmw.bfz
$ cd hpdgt
$ ls
52760 fcqrwd
54661 tzgt.hvh
$ cd ..
$ cd ..
$ cd tfc
$ ls
185481 bwntlh
18925 fcqrwd
$ cd ..
$ cd ..
$ cd ..
$ cd ldztz
$ ls
128430 bwz.fcz
dir dmpsnhdh
dir lbqgz
dir znrnj
$ cd dmpsnhdh
$ ls
238193 dncdssn.hdr
285939 hwfngq.dpw
$ cd ..
$ cd lbqgz
$ ls
171931 vgrp
$ cd ..
$ cd znrnj
$ ls
153738 vmwwbjqd
$ cd ..
$ cd ..
$ cd lmmw
$ ls
dir bqqnsfdj
163303 fcqrwd
43453 frqqdsr.hbf
33319 hjpmqrq
dir rlpcqtzg
$ cd bqqnsfdj
$ ls
dir bnl
2251 hjpmqrq
14707 rrqjwpm
dir tlnbvhdl
$ cd bnl
$ ls
33357 bnl.fqp
151237 bnl.vbs
40294 dmpsnhdh.hwz
76455 dncdssn.hdr
290341 hjpmqrq
dir lmw
dir nqw
$ cd lmw
$ ls
dir sfj
$ cd sfj
$ ls
156532 fcqrwd
$ cd ..
$ cd ..
$ cd nqw
$ ls
59928 dncdssn.hdr
$ cd ..
$ cd ..
$ cd tlnbvhdl
$ ls
183301 hjpmqrq
$ cd ..
$ cd ..
$ cd rlpcqtzg
$ ls
258638 dqt.mlc
$ cd ..
$ cd ..
$ cd wthvqw
$ ls
224501 pbb.zln
$ cd ..
$ cd zpdnprb
$ ls
dir bnl
dir ffg
dir jljlwpsv
212081 lrzc.lhj
dir rrqjwpm
dir twpq
dir vlgsrtm
$ cd bnl
$ ls
124009 hjgjf
74860 hjpmqrq
84996 lrdl.swf
dir pnzmp
$ cd pnzmp
$ ls
dir btbtlrs
128636 nfzf
$ cd btbtlrs
$ ls
107651 hhzbwd.wzj
$ cd ..
$ cd ..
$ cd ..
$ cd ffg
$ ls
57918 jwzbs.tnt
$ cd ..
$ cd jljlwpsv
$ ls
188175 dmpsnhdh.nnb
46693 fcqrwd
111557 pbb.zln
$ cd ..
$ cd rrqjwpm
$ ls
dir bftw
dir ccsfws
87225 mccw
290654 pbb.zln
147394 twzqc.pbz
52983 wsvgf
dir wwfgbzqh
$ cd bftw
$ ls
dir brl
167154 crs
dir lmw
dir rrqjwpm
dir twpq
174963 twpq.wjl
dir vnfhb
dir wcldzp
$ cd brl
$ ls
297937 wspcnp
$ cd ..
$ cd lmw
$ ls
166695 mcjql.jrv
$ cd ..
$ cd rrqjwpm
$ ls
198762 mwn
$ cd ..
$ cd twpq
$ ls
141835 jlwf.hcd
$ cd ..
$ cd vnfhb
$ ls
128626 tvmwhq.wfn
$ cd ..
$ cd wcldzp
$ ls
dir ncq
dir twpq
$ cd ncq
$ ls
dir wrtw
$ cd wrtw
$ ls
133331 fcqrwd
$ cd ..
$ cd ..
$ cd twpq
$ ls
151811 fcqrwd
$ cd ..
$ cd ..
$ cd ..
$ cd ccsfws
$ ls
100548 twpq.ppm
$ cd ..
$ cd wwfgbzqh
$ ls
dir lmw
dir mfms
dir pjbjgbcl
204154 qtflzwm
226500 vdmjj.htj
dir wzqbwr
$ cd lmw
$ ls
dir bgl
95150 dncdssn.hdr
119653 frqqdsr.hbf
97941 hjpmqrq
dir jqthwzj
$ cd bgl
$ ls
dir lmw
dir rrqjwpm
$ cd lmw
$ ls
233655 wmdldvbz
$ cd ..
$ cd rrqjwpm
$ ls
242918 frqqdsr.hbf
227581 hjpmqrq
dir hsvnmlp
dir nsch
25524 pbb.zln
dir qlgg
dir twpq
67453 twpq.fms
$ cd hsvnmlp
$ ls
264517 pbb.zln
$ cd ..
$ cd nsch
$ ls
7898 cmsdzh
233270 dmpsnhdh.bsq
101256 frl
133902 jzvh.vdv
dir lmw
dir sgjsg
130245 wcftvft
$ cd lmw
$ ls
69572 bnjnc.csp
$ cd ..
$ cd sgjsg
$ ls
38856 tnzpz.tbq
$ cd ..
$ cd ..
$ cd qlgg
$ ls
276013 frbstg.pzb
$ cd ..
$ cd twpq
$ ls
136454 fhwz.bqb
94099 rglp
114026 tsrt.cbd
26252 zhclpzm.rqf
$ cd ..
$ cd ..
$ cd ..
$ cd jqthwzj
$ ls
128200 lmw.btl
$ cd ..
$ cd ..
$ cd mfms
$ ls
274935 dmpsnhdh
76547 lchwq.dsd
215701 pbb.zln
dir rmwtvjt
$ cd rmwtvjt
$ ls
74490 hjpmqrq
$ cd ..
$ cd ..
$ cd pjbjgbcl
$ ls
231757 cjcpwwc.wbf
dir cswvftzs
dir jtvtg
dir lmw
dir tnctbjr
dir tqsrfhdr
$ cd cswvftzs
$ ls
dir dchqnbns
dir smf
$ cd dchqnbns
$ ls
94111 szl.hqs
$ cd ..
$ cd smf
$ ls
dir dlnsgvl
dir zglt
$ cd dlnsgvl
$ ls
dir dsz
$ cd dsz
$ ls
156473 hjpmqrq
$ cd ..
$ cd ..
$ cd zglt
$ ls
295383 frgg.sdp
$ cd ..
$ cd ..
$ cd ..
$ cd jtvtg
$ ls
202254 bftv.rqb
58419 lmw
$ cd ..
$ cd lmw
$ ls
8097 fcqrwd
$ cd ..
$ cd tnctbjr
$ ls
250830 frqqdsr.hbf
dir gzrcqr
$ cd gzrcqr
$ ls
dir fnzgsnv
$ cd fnzgsnv
$ ls
117215 hjpmqrq
$ cd ..
$ cd ..
$ cd ..
$ cd tqsrfhdr
$ ls
96381 lmw
$ cd ..
$ cd ..
$ cd wzqbwr
$ ls
149066 dmpsnhdh.vnd
dir dpbcgfdr
dir swp
14495 twpq.gsb
dir zhj
$ cd dpbcgfdr
$ ls
12909 dmpsnhdh
dir jvn
173491 mnhpr.lpr
222018 rfqfjmd.jqq
205077 wbbdrpr.hzj
dir wzpbbbhm
$ cd jvn
$ ls
117656 vqddrqlq.nfd
233109 vqqvh.swz
$ cd ..
$ cd wzpbbbhm
$ ls
143534 pbb.zln
$ cd ..
$ cd ..
$ cd swp
$ ls
131295 pbb.zln
$ cd ..
$ cd zhj
$ ls
166268 pbb.zln
33734 rrqjwpm.blg
$ cd ..
$ cd ..
$ cd ..
$ cd ..
$ cd twpq
$ ls
dir bppvlwqs
dir bvh
dir rmcdr
dir tdn
2230 whb.lfb
dir wwtwnvh
$ cd bppvlwqs
$ ls
195026 hvlhgsw
279259 rrqjwpm
$ cd ..
$ cd bvh
$ ls
dir lmw
$ cd lmw
$ ls
66958 pdqnd
$ cd ..
$ cd ..
$ cd rmcdr
$ ls
dir dmpsnhdh
182930 grj
dir pmrdhrth
119725 qpcqclqh
77890 sjgfjz
142855 twpq
dir zbmcrvbh
$ cd dmpsnhdh
$ ls
dir rrqjwpm
188474 zgjzpbl.vgv
$ cd rrqjwpm
$ ls
dir bnl
dir lmw
7598 vsntvs.pdv
$ cd bnl
$ ls
245600 lmw.mgf
$ cd ..
$ cd lmw
$ ls
73396 hjpmqrq
$ cd ..
$ cd ..
$ cd ..
$ cd pmrdhrth
$ ls
173155 rrqjwpm.pjw
178530 smgpzs.qtj
$ cd ..
$ cd zbmcrvbh
$ ls
124201 fcqrwd
135578 hjpmqrq
54356 hnztplsp.qlh
dir lmw
58350 pbb.zln
dir qfrvdm
dir rcg
15267 rwbzjpt.djn
$ cd lmw
$ ls
dir bbbll
28362 bfgfwlf.wvg
229637 dmpndms.fln
146121 dncdssn.hdr
131039 frqqdsr.hbf
152805 hjpmqrq
dir mlz
$ cd bbbll
$ ls
169940 dncdssn.hdr
216888 pbb.zln
248369 tjpmlr.vmf
$ cd ..
$ cd mlz
$ ls
115167 bhfv.fts
$ cd ..
$ cd ..
$ cd qfrvdm
$ ls
284564 pbb.zln
$ cd ..
$ cd rcg
$ ls
dir sqzjz
$ cd sqzjz
$ ls
116435 jrstpcpl.zsq
$ cd ..
$ cd ..
$ cd ..
$ cd ..
$ cd tdn
$ ls
143971 cmg
133317 fcqrwd
dir pstpclp
133161 tddv
$ cd pstpclp
$ ls
267351 hjpmqrq
86930 rrqjwpm.dvl
$ cd ..
$ cd ..
$ cd wwtwnvh
$ ls
256584 frqqdsr.hbf
114564 twpq.wrd
$ cd ..
$ cd ..
$ cd vlgsrtm
$ ls
148089 bnl.jzj
119796 cjfphsfw.hnd
197668 cpw
dir dmpsnhdh
dir fjsglr
dir lmw
dir lqgrft
9231 pltdltrs
dir rmdp
109777 rncfff.fll
dir vgjzqjpq
dir ztnqnfnq
$ cd dmpsnhdh
$ ls
dir lltnrdtv
dir scthsg
$ cd lltnrdtv
$ ls
179511 pcvmpz
90913 tbr
$ cd ..
$ cd scthsg
$ ls
dir rrqjwpm
$ cd rrqjwpm
$ ls
188629 fcqrwd
$ cd ..
$ cd ..
$ cd ..
$ cd fjsglr
$ ls
139754 fcqrwd
dir pnsjwfzc
$ cd pnsjwfzc
$ ls
113848 lmw
$ cd ..
$ cd ..
$ cd lmw
$ ls
54999 dmpsnhdh
dir ffhcf
251476 frqqdsr.hbf
dir jpgqspqw
198972 nhfclq.pbh
180380 nqmjnvc.fvr
dir pfsjwmbc
213768 rcvccgcd
$ cd ffhcf
$ ls
40478 svmwstq.sjj
$ cd ..
$ cd jpgqspqw
$ ls
22181 hjpmqrq
$ cd ..
$ cd pfsjwmbc
$ ls
dir bcvchw
$ cd bcvchw
$ ls
225892 bnl.nwc
$ cd ..
$ cd ..
$ cd ..
$ cd lqgrft
$ ls
dir rrqjwpm
dir twpq
$ cd rrqjwpm
$ ls
54786 fcqrwd
3053 tthhqjm.ntd
$ cd ..
$ cd twpq
$ ls
109355 bnl
dir lmw
dir mhgqt
301291 rrqjwpm.lrm
271233 twpq.srp
$ cd lmw
$ ls
dir lmw
dir lngbszqm
$ cd lmw
$ ls
139640 pbb.zln
$ cd ..
$ cd lngbszqm
$ ls
98279 mqvq.gsj
283599 rvjd.dvt
$ cd ..
$ cd ..
$ cd mhgqt
$ ls
208165 fcqrwd
$ cd ..
$ cd ..
$ cd ..
$ cd rmdp
$ ls
dir bqn
170956 fcqrwd
90954 snnttp.gld
$ cd bqn
$ ls
75628 hdrgbrpc
$ cd ..
$ cd ..
$ cd vgjzqjpq
$ ls
dir bnl
dir wpfw
$ cd bnl
$ ls
25911 pbb.zln
$ cd ..
$ cd wpfw
$ ls
247784 bzll.ltc
$ cd ..
$ cd ..
$ cd ztnqnfnq
$ ls
dir dtpzsrfc
214055 srgzhp.nlr
$ cd dtpzsrfc
$ ls
142652 bhgwj".to_string();
    let root = deduce_filesystem_from_input(puzzle_input);
    let answer = sum_of_sizes_under_limit(&root);
    println!("answer: {answer}");
    match size_of_best_directory_to_delete(&root) {
        Some(answer2) => {
            println!("answer2: {answer2}");
        }
        None => {
            assert!(false);
        }
    }
}