using System;
using System.Collections.Generic;
using System.Linq;
using System.Numerics;
using System.Text;
using System.Threading;

public enum JumpCondition
{
    not_zero,
    zero
}

public class BfOpCode
{
    
    public enum BfOpCodeType
    {
        _nop,
        _inc,
        _dec,
        _rt,
        _lt,
        _set,
        _inp,
        _out,
        _jmp,
        _load,
        _zstore,
    }
    public BfOpCodeType type;
    //C++ TO C# CONVERTER NOTE: Classes must be named in C#, so the following class has been named AnonymousClass:
    
        //C++ TO C# CONVERTER NOTE: Classes must be named in C#, so the following class has been named AnonymousClass2:
        
            public int value1;
        
        //C++ TO C# CONVERTER NOTE: Classes must be named in C#, so the following class has been named AnonymousClass3:
        
            public JumpCondition jmp_cond;
            public int jmp_index;
        
        //C++ TO C# CONVERTER NOTE: Classes must be named in C#, so the following class has been named AnonymousClass4:
        
            public Int32 rel_pos;
            public Int32 mult;
        
    
    public BfOpCode(BfOpCodeType type)
    {
        this.type = type;
        value1 = 0;
        jmp_index = 0;
        rel_pos = 0;
        mult = 0;
    } // inp, out, store
    public BfOpCode(BfOpCodeType type, int cell)
    {
        this.type = type;
        value1 = 0;
        jmp_index = 0;
        rel_pos = new Int32();
        mult = new Int32();
        this.value1 = cell;
    } // inc, dec, rt, lt, set
    public BfOpCode(BfOpCodeType type, JumpCondition cond, int index)
    {
        this.type = type;
        this.jmp_cond = cond;
        value1 = 0;
        jmp_index = 0;
        rel_pos = new Int32();
        mult = new Int32();
        this.jmp_index = index;
    } // jmp
    public BfOpCode(BfOpCodeType type, int rel_pos, int mult)
    {
        this.type = type;
        this.rel_pos = new Int32();
        this.rel_pos = rel_pos;
        jmp_index = new int();
        value1 = mult;
        this.mult = new Int32();
        this.mult = mult;
    } // load

    public BfOpCode()
    {
    }

    public BfOpCode Clone()
    {
        var val = new BfOpCode();

    val.value1= value1;
        val.type = type;

    val.jmp_cond = jmp_cond;
    val.jmp_index = jmp_index;


    val.rel_pos = rel_pos;
    val.mult = rel_pos;
        return val;
    }

    public override string ToString()
    {
        return Enum.GetName(typeof(BfOpCodeType), type) + value1;
    }
}

public class BfTransducer
{
    public string data;
    public int len = new int();
    public List<BfOpCode> code;
    public BfTransducer(byte[] raw)
    {
        data = Encoding.UTF8.GetString(raw);
        //C++ TO C# CONVERTER TODO TASK: The following line was determined to be a copy assignment (rather than a reference assignment) - this should be verified and a 'CopyFrom' method should be created:
        //ORIGINAL LINE: this.len = (int)raw.size();
        this.len = data.Length;
        code = new List<BfOpCode>();
    }
    public void push(BfOpCode c)
    {
        code.Add(c);
    }
    public int transduce()
    {
        int incr = 0;
        int shift = 0;
        Stack<int> indices = new Stack<int>();
        int top = new int();

        data = data.Replace("[+]", "*").Replace("[-]","*");
        foreach(char c in data)
        {
            switch (c)
            {
                case '+':
                    incr++;
                    break;
                case '-':
                    incr--;
                    break;
                default:
                    if (incr != 0)
                    {
                        push(new BfOpCode(incr > 0 ? BfOpCode.BfOpCodeType._inc : BfOpCode.BfOpCodeType._dec, (int)Math.Abs(incr)));
                    }
                    incr = 0;
                    break;
            }
            switch (c)
            {
                case '>':
                    shift++;
                    break;
                case '<':
                    shift--;
                    break;
                default:
                    if (shift != 0)
                    {
                        push(new BfOpCode(shift > 0 ? BfOpCode.BfOpCodeType._rt : BfOpCode.BfOpCodeType._lt, Math.Abs(shift)));
                    }
                    shift = 0;
                    break;
            }
            switch (c)
            {
                case '[':
                    code.Add(new BfOpCode{type = BfOpCode.BfOpCodeType._jmp, jmp_cond = JumpCondition.zero, jmp_index = int.MaxValue});
                    indices.Push((int)code.Count);
                    break;
                case ']':
                    if (indices.Count==0)
                    {
                        Console.WriteLine("error: unassigned closure");
                        return -1;
                    }
                    top = indices.Pop();
                    code.Add(new BfOpCode{type = BfOpCode.BfOpCodeType._jmp, jmp_cond = JumpCondition.not_zero, jmp_index = top});
                    code[top - 1].jmp_index = (int)code.Count;
                    break;
                case '*':
                    push(new BfOpCode(BfOpCode.BfOpCodeType._set, 0));
                    break;
                case '.':
                    push(new BfOpCode(BfOpCode.BfOpCodeType._out));
                    break;
                case ',':
                    push(new BfOpCode(BfOpCode.BfOpCodeType._inp));
                    break;
            }
        }
        return 0;
    }
    public int optimize()
    {
        int startindex = new int();
        bool in_loop = false;
        int shifts = 0;
        Dictionary<int, int> mult = new Dictionary<int, int>();
        Dictionary<int, int> set = new Dictionary<int, int>();
        for (int i = 0; i < code.Count; i++)
        {
            /*if(i==1383) {
				int dennisistdumm = true;
			}*/
            var op_code = code[i];
            if (op_code.type == BfOpCode.BfOpCodeType._jmp && op_code.jmp_cond == JumpCondition.zero)
            {
                in_loop = true;
                //C++ TO C# CONVERTER TODO TASK: The following line was determined to be a copy assignment (rather than a reference assignment) - this should be verified and a 'CopyFrom' method should be created:
                //ORIGINAL LINE: startindex = i;
                startindex = (i);
                shifts = 0;
                mult = new Dictionary<int, int>();
                set = new Dictionary<int, int>();
                continue;
            }
            if (in_loop)
            {
                switch (op_code.type)
                {
                    case BfOpCode.BfOpCodeType._inp:
                    case BfOpCode.BfOpCodeType._out:
                        in_loop = false;
                        continue;
                    case BfOpCode.BfOpCodeType._lt:
                        shifts -= op_code.value1;
                        break;
                    case BfOpCode.BfOpCodeType._rt:
                        shifts += op_code.value1;
                        break;
                    case BfOpCode.BfOpCodeType._inc:
                        if (!mult.ContainsKey(shifts))
                            mult.Add(shifts, 0);
                        mult[shifts] += op_code.value1;
                        break;
                    case BfOpCode.BfOpCodeType._set:
                        if (!set.ContainsKey(shifts))
                            set.Add(shifts, 0);
                        set[shifts] = 0;
                        break;
                    case BfOpCode.BfOpCodeType._dec:
                        if(!mult.ContainsKey(shifts))
                            mult.Add(shifts, 0);
                        mult[shifts] -= op_code.value1;
                        break;
                    case BfOpCode.BfOpCodeType._jmp when op_code.jmp_cond == JumpCondition.not_zero:
                        mult.TryGetValue(0, out int m);
                        if (shifts != 0 || !mult.ContainsKey(0))
                        {
                            in_loop = false;
                            continue;
                        }
                        if (m == 0)
                        {
                            Console.WriteLine("warning: infinite loop detected");
                        }
                        if (m != -1)
                        {
                            in_loop = false;
                            continue;
                        }
                        int n = startindex+1;
                        code[n++] = new BfOpCode(BfOpCode.BfOpCodeType._zstore);
                        //code[n++] = new BfOpCode(BfOpCode.BfOpCodeType._set, 0);
                        foreach (var j in mult)
                        {
                            if (j.Key == 0 || j.Value == 0)
                            {
                                continue;
                            }
                            code[n++] = new BfOpCode(BfOpCode.BfOpCodeType._load, j.Key, j.Value);
                        }
                        foreach (var j in set)
                        {
                            if (j.Key == 0)
                            {
                                continue;
                            }
                            code[n++] = new BfOpCode(BfOpCode.BfOpCodeType._set, j.Key, 0);
                        }
                        for (; n <= i; n++)
                        {
                            code[n] = new BfOpCode(BfOpCode.BfOpCodeType._nop);
                        }
                        code[n-1] = new BfOpCode(BfOpCode.BfOpCodeType._jmp){jmp_cond = JumpCondition.zero, jmp_index = n};
                        in_loop = false;
                        break;
                }
            }
        }
        return 0;
    }
    public void print()
    {
        for (int i = 0; i < code.Count; i++)
        {
            var c = code[i];
            Console.Write("{0,4:D}: ", i);
            printOpCode(c);
        }
    }

    public static void printOpCode(BfOpCode c)
    {
        switch (c.type)
        {
            case BfOpCode.BfOpCodeType._inc:
                Console.Write("+ {0:D}\n", c.value1);
                break;
            case BfOpCode.BfOpCodeType._dec:
                Console.Write("- {0:D}\n", c.value1);
                break;
            case BfOpCode.BfOpCodeType._rt:
                Console.Write("> {0:D}\n", c.value1);
                break;
            case BfOpCode.BfOpCodeType._lt:
                Console.Write("< {0:D}\n", c.value1);
                break;
            //C++ TO C# CONVERTER TODO TASK: C# does not allow fall-through from a non-empty 'case':
            case BfOpCode.BfOpCodeType._nop:
                Console.WriteLine("");
                break;
            case BfOpCode.BfOpCodeType._set:
                Console.Write("= {0:D}\n", c.value1);
                break;
            case BfOpCode.BfOpCodeType._inp:
                Console.Write("cin<<\n");
                break;
            case BfOpCode.BfOpCodeType._out:
                Console.Write("cout>>\n");
                break;
            case BfOpCode.BfOpCodeType._jmp:
                Console.Write("=> {0}0 | {1:D}\n", c.jmp_cond == (JumpCondition.zero) ? '=' : '!', c.jmp_index);
                break;
            case BfOpCode.BfOpCodeType._load:
                Console.Write("x>>[{0:D}]*{1:D}\n", c.rel_pos, c.mult);
                break;
            case BfOpCode.BfOpCodeType._zstore:
                Console.Write("x<<\n");
                break;
        }
    } 
    public int join()
    {
        // ToDo: remove nops
        return -1;
    }
}

public class BfVirtualEnv
{
    public List<BfOpCode> code, code1;
    public int len = new int();
    public BfVirtualEnv(List<BfOpCode> code, List<BfOpCode> code1)
    {
        //this.code = new BfOpCode(code);
        //C++ TO C# CONVERTER TODO TASK: The following line was determined to be a copy assignment (rather than a reference assignment) - this should be verified and a 'CopyFrom' method should be created:
        //ORIGINAL LINE: this.len = (int)code.size();
        this.code = new List<BfOpCode>(code);
        this.code1 = new List<BfOpCode>(code1);
        this.len = code.Count;
    }
    //C++ TO C# CONVERTER WARNING: 'const' methods are not available in C#:
    //ORIGINAL LINE: int run() const
    public int run()
    {
        int[] memory= new int[4096], memory1 = new int[4096];
        //C++ TO C# CONVERTER TODO TASK: The memory management function 'memset' has no equivalent in C#:
        //memset(memory, 0, sizeof(int));
        int cell = 0, cell1 = 0;
        int storeval=0, storeval1=0;
        int i = 0, j = 0;

        int lines_executed = 0;
        while (i < len && j<len )
        {
            
            //DoStep(ref i, ref memory, ref cell, ref code, ref storeval, lines_executed>4214200000, "Vanilla");
            DoStep(ref j, ref memory1, ref cell1, ref code1, ref storeval1, lines_executed >4214200000, "Optimized");

            /*bool same = true;
            for (var index = 0; index < memory.Length; index++)
            {
                int i1 = memory[index];
                int i2 = memory1[index];
                if (i1 != i2||i1<0||i2<0)
                    same = false;
            }

            if (!same||i!=j||cell1!=cell)
            {
                Console.WriteLine($"{i},{j}, memory divergiert");
            }
*/

            lines_executed++;
            /*i++;
            j++;*/

        }
        return 0;

        
    }

    void DoStep(ref int i, ref int[] memory, ref int cell_p, ref List<BfOpCode> code, ref int storeval, bool debug, string name)
    {
        bool reached_jpm = false;
        if (debug)
        {
            Console.WriteLine(name + ":");
        }

        do
        {
            BfOpCode c = code[i];
            BfOpCode.BfOpCodeType ct = c.type;
            // printf("%i", i);
            // if (ct == BfOpCode::_jmp) printf(" %i", *cell_p);
            //puts("");
            if (debug)
            {
                Console.Write(i+":");
                BfTransducer.printOpCode(c);
            }

            switch (ct)
            {
                case BfOpCode.BfOpCodeType._inc:
                    memory[cell_p] += c.value1;
                    break;
                case BfOpCode.BfOpCodeType._dec:
                    memory[cell_p] -= c.value1;
                    break;
                case BfOpCode.BfOpCodeType._rt:
                    cell_p += c.value1;
                    break;
                case BfOpCode.BfOpCodeType._lt:
                    cell_p -= c.value1;
                    break;
                case BfOpCode.BfOpCodeType._set:
                    memory[cell_p + c.rel_pos] = c.value1;
                    break;
                case BfOpCode.BfOpCodeType._zstore:
                    //C++ TO C# CONVERTER TODO TASK: The following line was determined to be a copy assignment (rather than a reference assignment) - this should be verified and a 'CopyFrom' method should be created:
                    //ORIGINAL LINE: storeval = *cell_p;
                    storeval = memory[cell_p];
                    memory[cell_p] = 0;
                    break;
                case BfOpCode.BfOpCodeType._load:
                    memory[cell_p + c.rel_pos] += storeval * c.mult;
                    break;
                case BfOpCode.BfOpCodeType._jmp:
                    if (!((c.jmp_cond == JumpCondition.zero) ^ (memory[cell_p] == 0)))
                    {
                        i = c.jmp_index - 1;
                        
                    }
                    if(memory[cell_p] == 0) { reached_jpm = true; }

                    break;
                case BfOpCode.BfOpCodeType._out:
                    Console.Write((char) memory[cell_p]);
                    break;
                case BfOpCode.BfOpCodeType._inp:
                    memory[cell_p] = (int) Console.Read();
                    break;
            }

            i++;
        } while (!reached_jpm);
    }
}
class Program
{
    static void Main(string[] args)
    {
        if (args.Length <= 0)
        {
            Console.WriteLine("interfuck is an (more or less) optimized brainfuck interpreter");
            Console.WriteLine("run it as follows: `interfuck script.bf`");
        }
        else if (args.Length >= 1)
        {
            var raw = System.IO.File.ReadAllBytes(args[0]);
            BfTransducer duc = new BfTransducer(raw);
            if (duc.transduce() == 1)
            {
                Console.WriteLine("error: error in bf file");
            }
            var duct1 = new BfTransducer(raw);
            duct1.code = duc.code.Select(x => x.Clone()).ToList();

            if (duct1.optimize() == 1)
            {
                Console.WriteLine("error: error while optimizing bf file");
            }
            //duc.print();
            var sw = System.Diagnostics.Stopwatch.StartNew();
            BfVirtualEnv env = new BfVirtualEnv(duc.code, duct1.code);
            if (env.run() == 1)
            {
                Console.WriteLine("error: error while executing code in the virtual env");
            }
            //fclose(fhandle);
            Console.WriteLine(sw.ElapsedMilliseconds);
        }

        Console.ReadLine();
    }
}