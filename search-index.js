var N=null,E="",T="t",U="u",searchIndex={};
var R=["day02::initial","result","description","solution_part1","solution_part2","Day02Initial","error","parse_input","initial","benchmark","to_benchmark","benchmarkvector","BenchmarkVector","PUZZLE_INPUT","SolutionPart1","SolutionPart2","try_from","try_into","borrow_mut","day03::initial","type_id","day03entry","formatter","Day03Entry","Day03Initial","Context","Provides the `context` method for `Result`.","Wrap the error value with additional context.","with_context","Wrap the error value with additional context that is…","`Result<T, Error>`","day04input","borrow","typeid","day04::initial","password","Day04Input","Password","Day04Initial","context","__shrinkwrap_t","Map a function over the wrapped value, consuming it in the…","Map a function over the wrapped value without consuming it.","Map a function over the wrapped value, potentially…","__shrinkwrap_f","map_ref","day06inversemap","to_owned","clone_into","day06::initial","day06::path_diff","hashmap","day06map","orbiter","orbiting","deref_mut","Day06InverseMap","Day06Map","Orbiting","Day06Initial","Day06PathDiff","day01::initial","day01entry","Day01Part2WeightIterator","Day01Entry","Day01Initial","as_ref"];
searchIndex["day01"]={"doc":"Day 01: The Tyranny of the Rocket Equation","i":[[8,R[25],"day01",R[26],N,N],[10,R[39],E,R[27],0,[[["c"]],[[R[6]],[R[1],[R[6]]]]]],[10,R[28],E,R[29],0,[[["f"]],[[R[6]],[R[1],[R[6]]]]]],[6,"Result",E,R[30],N,N],[3,R[64],E,E,N,N],[5,"weight",E,E,N,[[["usize"]],["usize"]]],[5,R[7],E,E,N,[[["str"]]]],[0,R[8],E,E,N,N],[3,R[65],R[61],E,N,N],[3,R[63],E,E,N,N],[11,"new",E,E,1,[[[R[62]]],["day01part2weightiterator"]]],[0,R[9],"day01",E,N,N],[5,R[10],"day01::benchmark",E,N,[[],[R[11]]]],[6,R[12],E,E,N,N],[7,R[13],"day01",E,N,N],[8,"AoC",E,E,N,N],[16,R[14],E,E,2,N],[16,R[15],E,E,2,N],[11,R[2],E,E,2,[[["self"]],["str"]]],[10,"new",E,E,2,[[["str"]],["self"]]],[11,R[3],E,E,2,[[["self"]]]],[11,R[4],E,E,2,[[["self"]]]],[11,"map",E,R[41],3,[[["fnmut"]],[R[40]]]],[11,R[45],E,R[42],3,[[["self"],["fnmut"]],[R[40]]]],[11,"weight",E,E,3,[[["self"]],["usize"]]],[11,"into",E,E,3,[[],[U]]],[11,"from",E,E,3,[[[T]],[T]]],[11,R[16],E,E,3,[[[U]],[R[1]]]],[11,R[17],E,E,3,[[],[R[1]]]],[11,R[18],E,E,3,[[["self"]],[T]]],[11,R[32],E,E,3,[[["self"]],[T]]],[11,R[20],E,E,3,[[["self"]],[R[33]]]],[11,"into",R[61],E,4,[[],[U]]],[11,"from",E,E,4,[[[T]],[T]]],[11,R[16],E,E,4,[[[U]],[R[1]]]],[11,R[17],E,E,4,[[],[R[1]]]],[11,R[18],E,E,4,[[["self"]],[T]]],[11,R[32],E,E,4,[[["self"]],[T]]],[11,R[20],E,E,4,[[["self"]],[R[33]]]],[11,"into",E,E,1,[[],[U]]],[11,"into_iter",E,E,1,[[],["i"]]],[11,"from",E,E,1,[[[T]],[T]]],[11,R[16],E,E,1,[[[U]],[R[1]]]],[11,R[17],E,E,1,[[],[R[1]]]],[11,R[18],E,E,1,[[["self"]],[T]]],[11,R[32],E,E,1,[[["self"]],[T]]],[11,R[20],E,E,1,[[["self"]],[R[33]]]],[11,R[2],E,E,4,[[["self"]],["str"]]],[11,"new",E,E,4,[[["str"]],["day01initial"]]],[11,R[3],E,E,4,[[["self"]]]],[11,R[4],E,E,4,[[["self"]]]],[11,R[66],"day01",E,3,[[["self"]],["usize"]]],[11,"eq",E,E,3,[[["self"],[R[62]]],["bool"]]],[11,"ne",E,E,3,[[["self"],[R[62]]],["bool"]]],[11,"next",R[61],E,1,[[["self"]],["option"]]],[11,"deref","day01",E,3,[[["self"]]]],[11,"fmt",R[61],E,4,[[["self"],[R[22]]],[R[1]]]],[11,"fmt",E,E,1,[[["self"],[R[22]]],[R[1]]]],[11,"fmt","day01",E,3,[[["self"],[R[22]]],[R[1]]]],[11,R[32],E,E,3,[[["self"]],["usize"]]]],"p":[[8,R[25]],[3,R[63]],[8,"AoC"],[3,R[64]],[3,R[65]]]};
searchIndex["day02"]={"doc":"Day 02: 1202 Program Alarm","i":[[5,R[7],"day02",E,N,[[["str"]]]],[0,R[8],E,E,N,N],[3,R[5],R[0],E,N,N],[11,"state",E,E,0,[[["self"]]]],[11,"step",E,E,0,[[["self"]]]],[11,"run",E,E,0,[[["self"]]]],[0,R[9],"day02",E,N,N],[5,R[10],"day02::benchmark",E,N,[[],[R[11]]]],[6,R[12],E,E,N,N],[6,"OpCode","day02",E,N,N],[7,R[13],E,E,N,N],[17,"OPCODE_ADD",E,E,N,N],[17,"OPCODE_MUL",E,E,N,N],[17,"OPCODE_DONE",E,E,N,N],[17,"OPCODE_JUMP",E,E,N,N],[17,"IDX_OUTPUT",E,E,N,N],[17,"IDX_NOUN",E,E,N,N],[17,"IDX_VERB",E,E,N,N],[17,"PART2_EXPECTED_VALUE",E,E,N,N],[8,"AoC",E,E,N,N],[16,R[14],E,E,1,N],[16,R[15],E,E,1,N],[11,R[2],E,E,1,[[["self"]],["str"]]],[10,"new",E,E,1,[[["str"]],["self"]]],[11,R[3],E,E,1,[[["self"]]]],[11,R[4],E,E,1,[[["self"]]]],[11,"into",R[0],E,0,[[],[U]]],[11,"from",E,E,0,[[[T]],[T]]],[11,R[16],E,E,0,[[[U]],[R[1]]]],[11,R[17],E,E,0,[[],[R[1]]]],[11,R[18],E,E,0,[[["self"]],[T]]],[11,R[32],E,E,0,[[["self"]],[T]]],[11,R[20],E,E,0,[[["self"]],[R[33]]]],[11,R[2],E,E,0,[[["self"]],["str"]]],[11,"new",E,E,0,[[["str"]],["day02initial"]]],[11,R[3],E,E,0,[[["self"]]]],[11,R[4],E,E,0,[[["self"]]]],[11,"fmt",E,E,0,[[["self"],[R[22]]],[R[1]]]]],"p":[[3,R[5]],[8,"AoC"]]};
searchIndex["day03"]={"doc":"Day 03: Crossed Wires","i":[[8,R[25],"day03",R[26],N,N],[10,R[39],E,R[27],0,[[["c"]],[[R[6]],[R[1],[R[6]]]]]],[10,R[28],E,R[29],0,[[["f"]],[[R[6]],[R[1],[R[6]]]]]],[6,"Result",E,R[30],N,N],[4,R[23],E,E,N,N],[13,"Up",E,E,1,N],[13,"Down",E,E,1,N],[13,"Left",E,E,1,N],[13,"Right",E,E,1,N],[5,"parse_line",E,E,N,[[["str"]]]],[5,R[7],E,E,N,[[["str"]],[["vec",["vec"]],["vec",[R[21]]]]]],[0,R[8],E,E,N,N],[3,R[24],R[19],E,N,N],[0,R[9],"day03",E,N,N],[5,R[10],"day03::benchmark",E,N,[[],[R[11]]]],[6,R[12],E,E,N,N],[6,"Length","day03",E,N,N],[7,R[13],E,E,N,N],[8,"AoC",E,E,N,N],[16,R[14],E,E,2,N],[16,R[15],E,E,2,N],[11,R[2],E,E,2,[[["self"]],["str"]]],[10,"new",E,E,2,[[["str"]],["self"]]],[11,R[3],E,E,2,[[["self"]]]],[11,R[4],E,E,2,[[["self"]]]],[11,"into",E,E,1,[[],[U]]],[11,"from",E,E,1,[[[T]],[T]]],[11,R[16],E,E,1,[[[U]],[R[1]]]],[11,R[17],E,E,1,[[],[R[1]]]],[11,R[18],E,E,1,[[["self"]],[T]]],[11,R[32],E,E,1,[[["self"]],[T]]],[11,R[20],E,E,1,[[["self"]],[R[33]]]],[11,"into",R[19],E,3,[[],[U]]],[11,"from",E,E,3,[[[T]],[T]]],[11,R[16],E,E,3,[[[U]],[R[1]]]],[11,R[17],E,E,3,[[],[R[1]]]],[11,R[18],E,E,3,[[["self"]],[T]]],[11,R[32],E,E,3,[[["self"]],[T]]],[11,R[20],E,E,3,[[["self"]],[R[33]]]],[11,R[2],E,E,3,[[["self"]],["str"]]],[11,"new",E,E,3,[[["str"]],["day03initial"]]],[11,R[3],E,E,3,[[["self"]]]],[11,R[4],E,E,3,[[["self"]]]],[11,"eq","day03",E,1,[[["self"],[R[21]]],["bool"]]],[11,"ne",E,E,1,[[["self"],[R[21]]],["bool"]]],[11,"fmt",R[19],E,3,[[["self"],[R[22]]],[R[1]]]],[11,"fmt","day03",E,1,[[["self"],[R[22]]],[R[1]]]],[11,"from_str",E,E,1,[[["str"]],[R[1]]]]],"p":[[8,R[25]],[4,R[23]],[8,"AoC"],[3,R[24]]]};
searchIndex["day04"]={"doc":"Day 04: Secure Container","i":[[8,R[25],"day04",R[26],N,N],[10,R[39],E,R[27],0,[[["c"]],[[R[6]],[R[1],[R[6]]]]]],[10,R[28],E,R[29],0,[[["f"]],[[R[6]],[R[1],[R[6]]]]]],[6,"Result",E,R[30],N,N],[3,R[36],E,E,N,N],[3,R[37],E,E,N,N],[5,R[7],E,E,N,[[["str"]],[[R[1],[R[31]]],[R[31]]]]],[0,R[8],E,E,N,N],[3,R[38],R[34],E,N,N],[0,R[9],"day04",E,N,N],[5,R[10],"day04::benchmark",E,N,[[],[R[11]]]],[6,R[12],E,E,N,N],[7,R[13],"day04",E,N,N],[8,"AoC",E,E,N,N],[16,R[14],E,E,1,N],[16,R[15],E,E,1,N],[11,R[2],E,E,1,[[["self"]],["str"]]],[10,"new",E,E,1,[[["str"]],["self"]]],[11,R[3],E,E,1,[[["self"]]]],[11,R[4],E,E,1,[[["self"]]]],[11,"range",E,E,2,[[["self"]],[["usize"],["range",["usize"]]]]],[11,"is_valid_part1",E,E,3,[[["self"]],["bool"]]],[11,"is_valid_part2",E,E,3,[[["self"]],["bool"]]],[11,"into",E,E,2,[[],[U]]],[11,"from",E,E,2,[[[T]],[T]]],[11,R[16],E,E,2,[[[U]],[R[1]]]],[11,R[17],E,E,2,[[],[R[1]]]],[11,R[18],E,E,2,[[["self"]],[T]]],[11,R[32],E,E,2,[[["self"]],[T]]],[11,R[20],E,E,2,[[["self"]],[R[33]]]],[11,R[47],E,E,3,[[["self"]],[T]]],[11,R[48],E,E,3,[[["self"],[T]]]],[11,"into",E,E,3,[[],[U]]],[11,"from",E,E,3,[[[T]],[T]]],[11,R[16],E,E,3,[[[U]],[R[1]]]],[11,R[17],E,E,3,[[],[R[1]]]],[11,R[18],E,E,3,[[["self"]],[T]]],[11,R[32],E,E,3,[[["self"]],[T]]],[11,R[20],E,E,3,[[["self"]],[R[33]]]],[11,"into",R[34],E,4,[[],[U]]],[11,"from",E,E,4,[[[T]],[T]]],[11,R[16],E,E,4,[[[U]],[R[1]]]],[11,R[17],E,E,4,[[],[R[1]]]],[11,R[18],E,E,4,[[["self"]],[T]]],[11,R[32],E,E,4,[[["self"]],[T]]],[11,R[20],E,E,4,[[["self"]],[R[33]]]],[11,R[2],E,E,4,[[["self"]],["str"]]],[11,"new",E,E,4,[[["str"]],["day04initial"]]],[11,R[3],E,E,4,[[["self"]]]],[11,R[4],E,E,4,[[["self"]]]],[11,"clone","day04",E,3,[[["self"]],[R[35]]]],[11,"eq",E,E,2,[[["self"],[R[31]]],["bool"]]],[11,"ne",E,E,2,[[["self"],[R[31]]],["bool"]]],[11,"eq",E,E,3,[[["self"],[R[35]]],["bool"]]],[11,"ne",E,E,3,[[["self"],[R[35]]],["bool"]]],[11,"fmt",R[34],E,4,[[["self"],[R[22]]],[R[1]]]],[11,"fmt","day04",E,2,[[["self"],[R[22]]],[R[1]]]],[11,"fmt",E,E,3,[[["self"],[R[22]]],[R[1]]]],[11,R[16],E,E,3,[[["str"]],[R[1]]]]],"p":[[8,R[25]],[8,"AoC"],[3,R[36]],[3,R[37]],[3,R[38]]]};
searchIndex["day06"]={"doc":"Day 06: Universal Orbit Map","i":[[8,R[25],"day06",R[26],N,N],[10,R[39],E,R[27],0,[[["c"]],[[R[6]],[R[1],[R[6]]]]]],[10,R[28],E,R[29],0,[[["f"]],[[R[6]],[R[1],[R[6]]]]]],[6,"Result",E,R[30],N,N],[3,R[56],E,E,N,N],[12,"0",E,E,1,N],[3,R[57],E,E,N,N],[12,"0",E,E,2,N],[3,"Orbiter",E,E,N,N],[3,R[58],E,E,N,N],[5,"parse_input_part1",E,E,N,[[["str"]],[R[46]]]],[5,"parse_input_part2",E,E,N,[[["str"]]]],[0,R[8],E,E,N,N],[3,R[59],R[49],E,N,N],[0,"path_diff","day06",E,N,N],[3,R[60],R[50],E,N,N],[0,R[9],"day06",E,N,N],[5,R[10],"day06::benchmark",E,N,[[],[R[11]]]],[6,R[12],E,E,N,N],[7,R[13],"day06",E,N,N],[8,"AoC",E,E,N,N],[16,R[14],E,E,3,N],[16,R[15],E,E,3,N],[11,R[2],E,E,3,[[["self"]],["str"]]],[10,"new",E,E,3,[[["str"]],["self"]]],[11,R[3],E,E,3,[[["self"]]]],[11,R[4],E,E,3,[[["self"]]]],[11,"map",E,R[41],1,[[["fnmut"]],[R[40]]]],[11,R[45],E,R[42],1,[[["self"],["fnmut"]],[R[40]]]],[11,"map_mut",E,R[43],1,[[["self"],[R[44]]],[R[40]]]],[11,"map",E,R[41],2,[[["fnmut"]],[R[40]]]],[11,R[45],E,R[42],2,[[["self"],["fnmut"]],[R[40]]]],[11,"map_mut",E,R[43],2,[[["self"],[R[44]]],[R[40]]]],[11,"map",E,R[41],4,[[["fnmut"]],[R[40]]]],[11,R[45],E,R[42],4,[[["self"],["fnmut"]],[R[40]]]],[11,"map",E,R[41],5,[[["fnmut"]],[R[40]]]],[11,R[45],E,R[42],5,[[["self"],["fnmut"]],[R[40]]]],[11,"new",E,E,1,[[],[R[46]]]],[11,"new",E,E,2,[[],[R[52]]]],[11,"into",E,E,1,[[],[U]]],[11,"from",E,E,1,[[[T]],[T]]],[11,R[16],E,E,1,[[[U]],[R[1]]]],[11,R[17],E,E,1,[[],[R[1]]]],[11,R[18],E,E,1,[[["self"]],[T]]],[11,R[32],E,E,1,[[["self"]],[T]]],[11,R[20],E,E,1,[[["self"]],[R[33]]]],[11,"into",E,E,2,[[],[U]]],[11,"from",E,E,2,[[[T]],[T]]],[11,R[16],E,E,2,[[[U]],[R[1]]]],[11,R[17],E,E,2,[[],[R[1]]]],[11,R[18],E,E,2,[[["self"]],[T]]],[11,R[32],E,E,2,[[["self"]],[T]]],[11,R[20],E,E,2,[[["self"]],[R[33]]]],[11,R[47],E,E,4,[[["self"]],[T]]],[11,R[48],E,E,4,[[["self"],[T]]]],[11,"into",E,E,4,[[],[U]]],[11,"from",E,E,4,[[[T]],[T]]],[11,R[16],E,E,4,[[[U]],[R[1]]]],[11,R[17],E,E,4,[[],[R[1]]]],[11,R[18],E,E,4,[[["self"]],[T]]],[11,R[32],E,E,4,[[["self"]],[T]]],[11,R[20],E,E,4,[[["self"]],[R[33]]]],[11,R[47],E,E,5,[[["self"]],[T]]],[11,R[48],E,E,5,[[["self"],[T]]]],[11,"into",E,E,5,[[],[U]]],[11,"from",E,E,5,[[[T]],[T]]],[11,R[16],E,E,5,[[[U]],[R[1]]]],[11,R[17],E,E,5,[[],[R[1]]]],[11,R[18],E,E,5,[[["self"]],[T]]],[11,R[32],E,E,5,[[["self"]],[T]]],[11,R[20],E,E,5,[[["self"]],[R[33]]]],[11,"into",R[49],E,6,[[],[U]]],[11,"from",E,E,6,[[[T]],[T]]],[11,R[16],E,E,6,[[[U]],[R[1]]]],[11,R[17],E,E,6,[[],[R[1]]]],[11,R[18],E,E,6,[[["self"]],[T]]],[11,R[32],E,E,6,[[["self"]],[T]]],[11,R[20],E,E,6,[[["self"]],[R[33]]]],[11,"into",R[50],E,7,[[],[U]]],[11,"from",E,E,7,[[[T]],[T]]],[11,R[16],E,E,7,[[[U]],[R[1]]]],[11,R[17],E,E,7,[[],[R[1]]]],[11,R[18],E,E,7,[[["self"]],[T]]],[11,R[32],E,E,7,[[["self"]],[T]]],[11,R[20],E,E,7,[[["self"]],[R[33]]]],[11,R[2],R[49],E,6,[[["self"]],["str"]]],[11,"new",E,E,6,[[["str"]],["day06initial"]]],[11,R[3],E,E,6,[[["self"]]]],[11,R[4],E,E,6,[[["self"]]]],[11,R[2],R[50],E,7,[[["self"]],["str"]]],[11,"new",E,E,7,[[["str"]],["day06pathdiff"]]],[11,R[3],E,E,7,[[["self"]]]],[11,R[4],E,E,7,[[["self"]]]],[11,"as_mut","day06",E,1,[[["self"]],[R[51]]]],[11,"as_mut",E,E,2,[[["self"]],[R[51]]]],[11,"clone",E,E,4,[[["self"]],[R[53]]]],[11,"clone",E,E,5,[[["self"]],[R[54]]]],[11,R[66],E,E,1,[[["self"]],[R[51]]]],[11,R[66],E,E,2,[[["self"]],[R[51]]]],[11,R[66],E,E,4,[[["self"]],["str"]]],[11,R[66],E,E,5,[[["self"]],["str"]]],[11,"eq",E,E,1,[[["self"],[R[46]]],["bool"]]],[11,"ne",E,E,1,[[["self"],[R[46]]],["bool"]]],[11,"eq",E,E,2,[[["self"],[R[52]]],["bool"]]],[11,"ne",E,E,2,[[["self"],[R[52]]],["bool"]]],[11,"eq",E,E,4,[[["self"],[R[53]]],["bool"]]],[11,"ne",E,E,4,[[["self"],[R[53]]],["bool"]]],[11,"eq",E,E,5,[[["self"],[R[54]]],["bool"]]],[11,"ne",E,E,5,[[["self"],[R[54]]],["bool"]]],[11,R[55],E,E,1,[[["self"]]]],[11,R[55],E,E,2,[[["self"]]]],[11,"hash",E,E,4,[[["self"],["__h"]]]],[11,"hash",E,E,5,[[["self"],["__h"]]]],[11,"deref",E,E,1,[[["self"]]]],[11,"deref",E,E,2,[[["self"]]]],[11,"deref",E,E,4,[[["self"]]]],[11,"deref",E,E,5,[[["self"]]]],[11,"fmt",R[49],E,6,[[["self"],[R[22]]],[R[1]]]],[11,"fmt",R[50],E,7,[[["self"],[R[22]]],[R[1]]]],[11,"fmt","day06",E,1,[[["self"],[R[22]]],[R[1]]]],[11,"fmt",E,E,2,[[["self"],[R[22]]],[R[1]]]],[11,"fmt",E,E,4,[[["self"],[R[22]]],[R[1]]]],[11,"fmt",E,E,5,[[["self"],[R[22]]],[R[1]]]],[11,R[18],E,E,1,[[["self"]],[R[51]]]],[11,R[18],E,E,2,[[["self"]],[R[51]]]],[11,R[32],E,E,1,[[["self"]],[R[51]]]],[11,R[32],E,E,2,[[["self"]],[R[51]]]],[11,R[32],E,E,4,[[["self"]],["str"]]],[11,R[32],E,E,5,[[["self"]],["str"]]]],"p":[[8,R[25]],[3,R[56]],[3,R[57]],[8,"AoC"],[3,"Orbiter"],[3,R[58]],[3,R[59]],[3,R[60]]]};
initSearch(searchIndex);addSearchOptions(searchIndex);