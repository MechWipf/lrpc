using System;
using System.Collections.Generic;
using System.Reflection;

namespace lrpc.val
{
    /// <summary>
    /// store and restore data queue
    /// </summary>
    public class ByteQue
    {
        private static Dictionary<Type, Store> STORES = new Dictionary<Type, Store>();
        static ByteQue()
        {
            STORES.Add(typeof(byte?), new ByteStore());
            STORES.Add(typeof(sbyte?), new SByteStore());
            STORES.Add(typeof(short?), new ShortStore());
            STORES.Add(typeof(ushort?), new UShortStore());
            STORES.Add(typeof(int?), new IntStore());
            STORES.Add(typeof(uint?), new UIntStore());
            STORES.Add(typeof(long?), new LongStore());
            STORES.Add(typeof(ulong?), new ULongStore());
            STORES.Add(typeof(float?), new FloatStore());
            STORES.Add(typeof(double?), new DoubleStore());
            STORES.Add(typeof(bool?), new BoolStore());
            STORES.Add(typeof(char?), new CharStore());
            STORES.Add(typeof(string), new StringStore());
        }

        /// <summary>
        /// register a custom data store
        /// </summary>
        /// <param name="type">store and restore data types</param>
        /// <param name="store">store and restore methods</param>
        public static void Regist(Type type, Store store)
        {
            STORES.Add(type, store);
        }

        private byte[] buff = new byte[16];
        private int head = 0;
        private int tail = 0;

        private void Push(byte val)
        {
            buff[tail] = val;
            if ((tail = tail + 1 & buff.Length - 1) == head)
            {
                byte[] a = new byte[buff.Length << 1];
                Array.Copy(buff, head, a, 0, buff.Length - head);
                Array.Copy(buff, 0, a, buff.Length - head, head);
                head = 0;
                tail = buff.Length;
                buff = a;
            }
        }

        private byte Pop()
        {
            if (head == tail)
            {
                return 0;
            }
            byte v = buff[head];
            head = head + 1 & buff.Length - 1;
            return v;
        }

        /// <summary>
        /// length
        /// </summary>
        public int Len
        {
            get
            {
                return tail - head & buff.Length - 1;
            }
        }

        /// <summary>
        /// the specified part of the array is added to the end of the queue
        /// </summary>
        /// <param name="arr">array containing data</param>
        /// <param name="six">starting index</param>
        /// <param name="eix">end index, excluding the element at this position</param>
        public void AddAll(byte[] arr, int six, int eix)
        {
            if (six < 0)
            {
                six = 0;
            }
            if (eix > arr.Length || eix < 0)
            {
                eix = arr.Length;
            }
            if (six < eix)
            {
                if (buff.Length - Len <= eix - six)
                {
                    int len = buff.Length;
                    while (len <= eix - six + Len)
                    {
                        len <<= 1;
                    }
                    byte[] a = new byte[len];
                    if (head < tail)
                    {
                        tail = Len;
                        Array.Copy(buff, head, a, 0, tail);
                    }
                    else if (head > tail)
                    {
                        Array.Copy(buff, head, a, 0, buff.Length - head);
                        Array.Copy(buff, 0, a, buff.Length - head, tail);
                        tail = Len;
                    }
                    else
                    {
                        tail = 0;
                    }
                    Array.Copy(arr, six, a, tail, eix - six);
                    head = 0;
                    tail += eix - six;
                    buff = a;
                }
                else
                {
                    if (tail < head || eix - six <= buff.Length - tail)
                    {
                        Array.Copy(arr, six, buff, tail, eix - six);
                        tail += eix - six;
                    }
                    else
                    {
                        Array.Copy(arr, six, buff, tail, buff.Length - tail);
                        Array.Copy(arr, buff.Length - tail, buff, 0, eix - six - buff.Length + tail);
                        tail += eix - six - buff.Length;
                    }
                }
            }
        }

        /// <summary>
        /// convert queue to byte array
        /// </summary>
        public byte[] ToArray()
        {
            byte[] a = new byte[Len];
            if (head < tail)
            {
                Array.Copy(buff, head, a, 0, a.Length);
            }
            else if (head > tail)
            {
                Array.Copy(buff, head, a, 0, buff.Length - head);
                Array.Copy(buff, 0, a, buff.Length - head, tail);
            }
            return a;
        }

        /// <summary>
        /// saved data length is not used for the value
        /// </summary>
        public void PushSize(int val)
        {
            for (int i = 0; i < 5; ++i)
            {
                if (val <= 0x7f && val >= 0)
                {
                    Push((byte)(val & 0x7f));
                    break;
                }
                else
                {
                    Push((byte)(val & 0x7f | 0x80));
                }
                val >>= 7;
            }
        }

        /// <summary>
        /// restore data length is not used for the value
        /// </summary>
        public int PopSize()
        {
            int s = 0;
            for (int i = 0; i < 5; ++i)
            {
                byte v = Pop();
                s |= (v & 0x7f) << 7 * i;
                if (v <= 0x7f)
                {
                    break;
                }
            }
            return s;
        }

        /// <summary>
        /// to save data to the queue, you need to have a parameterless constructor when using reflection
        /// </summary>
        /// <param name="type">type of data</param>
        /// <param name="val">data value</param>
        public void Push(Type type, object val)
        {
            if (type == typeof(byte))
            {
                Push((byte)val);
            }
            else if (type == typeof(sbyte))
            {
                Push((byte)(sbyte)val);
            }
            else if (type == typeof(short))
            {
                short v = (short)val;
                Push((byte)v);
                Push((byte)(v >> 8));
            }
            else if (type == typeof(ushort))
            {
                ushort v = (ushort)val;
                Push((byte)v);
                Push((byte)(v >> 8));
            }
            else if (type == typeof(int))
            {
                int v = (int)val;
                Push((byte)v);
                Push((byte)(v >> 8));
                Push((byte)(v >> 16));
                Push((byte)(v >> 24));
            }
            else if (type == typeof(uint))
            {
                uint v = (uint)val;
                Push((byte)v);
                Push((byte)(v >> 8));
                Push((byte)(v >> 16));
                Push((byte)(v >> 24));
            }
            else if (type == typeof(long))
            {
                long v = (long)val;
                Push((byte)v);
                Push((byte)(v >> 8));
                Push((byte)(v >> 16));
                Push((byte)(v >> 24));
                Push((byte)(v >> 32));
                Push((byte)(v >> 40));
                Push((byte)(v >> 48));
                Push((byte)(v >> 56));
            }
            else if (type == typeof(ulong))
            {
                ulong v = (ulong)val;
                Push((byte)v);
                Push((byte)(v >> 8));
                Push((byte)(v >> 16));
                Push((byte)(v >> 24));
                Push((byte)(v >> 32));
                Push((byte)(v >> 40));
                Push((byte)(v >> 48));
                Push((byte)(v >> 56));
            }
            else if (type == typeof(float))
            {
                byte[] v = BitConverter.GetBytes((float)val);
                Push(v[0]);
                Push(v[1]);
                Push(v[2]);
                Push(v[3]);
            }
            else if (type == typeof(double))
            {
                byte[] v = BitConverter.GetBytes((double)val);
                Push(v[0]);
                Push(v[1]);
                Push(v[2]);
                Push(v[3]);
                Push(v[4]);
                Push(v[5]);
                Push(v[6]);
                Push(v[7]);
            }
            else if (type == typeof(bool))
            {
                bool v = (bool)val;
                Push((byte)(v ? 1 : 0));
            }
            else if (type == typeof(char))
            {
                char v = (char)val;
                Push((byte)v);
                Push((byte)(v >> 8));
            }
            else if (STORES.ContainsKey(type))
            {
                STORES[type].Store(this, val);
            }
            else
            {
                FieldInfo[] flds = type.GetFields(BindingFlags.Instance | BindingFlags.Static | BindingFlags.Public | BindingFlags.NonPublic | BindingFlags.DeclaredOnly);
                foreach (FieldInfo fld in flds)
                {
                    Push(fld.FieldType, fld.GetValue(val));
                }
            }
        }

        /// <summary>
        /// to save data to the queue, you need to have a parameterless constructor when using reflection
        /// </summary>
        public void Push<T>(T val)
        {
            Push(typeof(T), val);
        }

        /// <summary>
        /// to restore data from the queue, you need a parameterless constructor when using reflection
        /// </summary>
        /// <param name="type">type of data to be restored</param>
        public object Pop(Type type)
        {
            if (type == typeof(byte))
            {
                return Pop();
            }
            if (type == typeof(sbyte))
            {
                return (sbyte)Pop();
            }
            if (type == typeof(short))
            {
                return (short)(Pop() & 0xff | Pop() << 8);
            }
            if (type == typeof(ushort))
            {
                return (ushort)(Pop() & 0xff | Pop() << 8);
            }
            if (type == typeof(int))
            {
                return Pop() & 0xff | (Pop() & 0xff) << 8 | (Pop() & 0xff) << 16 | Pop() << 24;
            }
            if (type == typeof(uint))
            {
                return (uint)(Pop() & 0xff | (Pop() & 0xff) << 8 | (Pop() & 0xff) << 16 | Pop() << 24);
            }
            if (type == typeof(long))
            {
                return ((long)Pop() & 0xff) | ((long)Pop() & 0xff) << 8 | ((long)Pop() & 0xff) << 16 | ((long)Pop() & 0xff) << 24 | ((long)Pop() & 0xff) << 32 | ((long)Pop() & 0xff) << 40 | ((long)Pop() & 0xff) << 48 | ((long)Pop() & 0xff) << 56;
            }
            if (type == typeof(ulong))
            {
                return ((ulong)Pop() & 0xff) | ((ulong)Pop() & 0xff) << 8 | ((ulong)Pop() & 0xff) << 16 | ((ulong)Pop() & 0xff) << 24 | ((ulong)Pop() & 0xff) << 32 | ((ulong)Pop() & 0xff) << 40 | ((ulong)Pop() & 0xff) << 48 | ((ulong)Pop() & 0xff) << 56;
            }
            if (type == typeof(float))
            {
                byte[] v = new byte[] { Pop(), Pop(), Pop(), Pop() };
                return BitConverter.ToSingle(v, 0);
            }
            if (type == typeof(double))
            {
                byte[] v = new byte[] { Pop(), Pop(), Pop(), Pop(), Pop(), Pop(), Pop(), Pop() };
                return BitConverter.ToDouble(v, 0);
            }
            if (type == typeof(bool))
            {
                return Pop() != 0;
            }
            if (type == typeof(char))
            {
                return (char)(Pop() & 0xff | Pop() << 8);
            }
            if (STORES.ContainsKey(type))
            {
                return STORES[type].Restore(this);
            }
            ConstructorInfo cons = type.GetConstructor(BindingFlags.Instance | BindingFlags.Static | BindingFlags.Public | BindingFlags.NonPublic | BindingFlags.DeclaredOnly, null, Type.EmptyTypes, null);
            object obj = cons.Invoke(null);
            FieldInfo[] flds = type.GetFields(BindingFlags.Instance | BindingFlags.Static | BindingFlags.Public | BindingFlags.NonPublic | BindingFlags.DeclaredOnly);
            foreach (FieldInfo fld in flds)
            {
                fld.SetValue(obj, Pop(fld.FieldType));
            }
            return obj;
        }

        /// <summary>
        /// to restore data from the queue, you need a parameterless constructor when using reflection
        /// </summary>
        public T Pop<T>()
        {
            return (T)Pop(typeof(T));
        }
    }
}
