using lrpc.val;

namespace lrpc.buf
{
    /// <summary>
    /// get actual length data
    /// </summary>
    public class RecvBuf
    {
        private ByteQue buff = new ByteQue();
        private int? size = null;

        /// <summary>
        /// add some data to the buffer until it reaches the specified length
        /// </summary>
        /// <param name="other">part of the data</param>
        /// <param name="length">actual data length</param>
        public void Append(byte[] other, int length)
        {
            if (length < 0)
            {
                length = other.Length;
            }
            if (size.HasValue)
            {
                if (size.Value > buff.Len)
                {
                    int l = size.Value - buff.Len;
                    if (l < length)
                    {
                        buff.AddAll(other, 0, l);
                    }
                    else
                    {
                        buff.AddAll(other, 0, length);
                    }
                }
            }
            else
            {
                if (buff.Len == 0)
                {
                    for (int x = 0; x < length; ++x)
                    {
                        if (x == 4 || other[x] <= 0x7f)
                        {
                            int s = 0;
                            for (int i = 0; i <= x; ++i)
                            {
                                s |= (other[i] & 0x7f) << 7 * i;
                            }
                            size = s;
                            s += x + 1;
                            if (s < length)
                            {
                                buff.AddAll(other, x + 1, s);
                            }
                            else
                            {
                                buff.AddAll(other, x + 1, length);
                            }
                            return;
                        }
                    }
                    buff.AddAll(other, 0, length);
                }
                else
                {
                    buff.AddAll(other, 0, length);
                    byte[] arr = buff.ToArray();
                    for (int x = 0; x < arr.Length; ++x)
                    {
                        if (x == 4 || arr[x] <= 0x7f)
                        {
                            int s = 0;
                            for (int i = 0; i <= x; ++i)
                            {
                                s |= (buff.Pop<byte>() & 0x7f) << 7 * i;
                            }
                            size = s;
                            s += x + 1;
                            if (arr.Length > s)
                            {
                                buff = new ByteQue();
                                buff.AddAll(arr, x + 1, s);
                            }
                            break;
                        }
                    }
                }
            }
        }

        /// <summary>
        /// the length of data that should be received
        /// </summary>
        public int? Size
        {
            get
            {
                return size;
            }
        }

        /// <summary>
        /// length of data received
        /// </summary>
        public int Len
        {
            get
            {
                return buff.Len;
            }
        }

        /// <summary>
        /// get ByteQue
        /// </summary>
        public ByteQue ByteQue
        {
            get
            {
                return buff;
            }
        }
    }
}
