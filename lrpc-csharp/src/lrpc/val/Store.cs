using System.Text;

namespace lrpc.val
{
    /// <summary>
    /// rules for storing and restoring data
    /// </summary>
    public interface Store
    {
        /// <summary>
        /// storing data
        /// </summary>
        /// <param name="que">queue for storing data</param>
        /// <param name="val">the data to be stored is not a class that implements this interface</param>
        void Store(ByteQue que, object val);
        /// <summary>
        /// restoring data
        /// </summary>
        /// <param name="que">queue for restoring data</param>
        /// <returns>previously stored data</returns>
        object Restore(ByteQue que);
    }

    class ByteStore : Store
    {
        public void Store(ByteQue que, object val)
        {
            if (val == null)
            {
                que.Push(false);
            }
            else
            {
                que.Push(true);
                que.Push((byte)val);
            }
        }

        public object Restore(ByteQue que)
        {
            if (que.Pop<bool>())
            {
                return que.Pop<byte>();
            }
            return null;
        }
    }

    class SByteStore : Store
    {
        public void Store(ByteQue que, object val)
        {
            if (val == null)
            {
                que.Push(false);
            }
            else
            {
                que.Push(true);
                que.Push((sbyte)val);
            }
        }

        public object Restore(ByteQue que)
        {
            if (que.Pop<bool>())
            {
                return que.Pop<sbyte>();
            }
            return null;
        }
    }

    class ShortStore : Store
    {
        public void Store(ByteQue que, object val)
        {
            if (val == null)
            {
                que.Push(false);
            }
            else
            {
                que.Push(true);
                que.Push((short)val);
            }
        }

        public object Restore(ByteQue que)
        {
            if (que.Pop<bool>())
            {
                return que.Pop<short>();
            }
            return null;
        }
    }

    class UShortStore : Store
    {
        public void Store(ByteQue que, object val)
        {
            if (val == null)
            {
                que.Push(false);
            }
            else
            {
                que.Push(true);
                que.Push((ushort)val);
            }
        }

        public object Restore(ByteQue que)
        {
            if (que.Pop<bool>())
            {
                return que.Pop<ushort>();
            }
            return null;
        }
    }

    class IntStore : Store
    {
        public void Store(ByteQue que, object val)
        {
            if (val == null)
            {
                que.Push(false);
            }
            else
            {
                que.Push(true);
                que.Push((int)val);
            }
        }

        public object Restore(ByteQue que)
        {
            if (que.Pop<bool>())
            {
                return que.Pop<int>();
            }
            return null;
        }
    }

    class UIntStore : Store
    {
        public void Store(ByteQue que, object val)
        {
            if (val == null)
            {
                que.Push(false);
            }
            else
            {
                que.Push(true);
                que.Push((uint)val);
            }
        }

        public object Restore(ByteQue que)
        {
            if (que.Pop<bool>())
            {
                return que.Pop<uint>();
            }
            return null;
        }
    }

    class LongStore : Store
    {
        public void Store(ByteQue que, object val)
        {
            if (val == null)
            {
                que.Push(false);
            }
            else
            {
                que.Push(true);
                que.Push((long)val);
            }
        }

        public object Restore(ByteQue que)
        {
            if (que.Pop<bool>())
            {
                return que.Pop<long>();
            }
            return null;
        }
    }

    class ULongStore : Store
    {
        public void Store(ByteQue que, object val)
        {
            if (val == null)
            {
                que.Push(false);
            }
            else
            {
                que.Push(true);
                que.Push((ulong)val);
            }
        }

        public object Restore(ByteQue que)
        {
            if (que.Pop<bool>())
            {
                return que.Pop<ulong>();
            }
            return null;
        }
    }

    class FloatStore : Store
    {
        public void Store(ByteQue que, object val)
        {
            if (val == null)
            {
                que.Push(false);
            }
            else
            {
                que.Push(true);
                que.Push((float)val);
            }
        }

        public object Restore(ByteQue que)
        {
            if (que.Pop<bool>())
            {
                return que.Pop<float>();
            }
            return null;
        }
    }

    class DoubleStore : Store
    {
        public void Store(ByteQue que, object val)
        {
            if (val == null)
            {
                que.Push(false);
            }
            else
            {
                que.Push(true);
                que.Push((double)val);
            }
        }

        public object Restore(ByteQue que)
        {
            if (que.Pop<bool>())
            {
                return que.Pop<double>();
            }
            return null;
        }
    }

    class BoolStore : Store
    {
        public void Store(ByteQue que, object val)
        {
            if (val == null)
            {
                que.Push(false);
            }
            else
            {
                que.Push(true);
                que.Push((bool)val);
            }
        }

        public object Restore(ByteQue que)
        {
            if (que.Pop<bool>())
            {
                return que.Pop<bool>();
            }
            return null;
        }
    }

    class CharStore : Store
    {
        public void Store(ByteQue que, object val)
        {
            if (val == null)
            {
                que.Push(false);
            }
            else
            {
                que.Push(true);
                que.Push((char)val);
            }
        }

        public object Restore(ByteQue que)
        {
            if (que.Pop<bool>())
            {
                return que.Pop<char>();
            }
            return null;
        }
    }

    class StringStore : Store
    {
        public void Store(ByteQue que, object val)
        {
            if (val == null)
            {
                que.Push(false);
            }
            else
            {
                que.Push(true);
                byte[] arr = Encoding.UTF8.GetBytes((string)val);
                que.PushSize(arr.Length);
                foreach (byte ch in arr)
                {
                    que.Push(ch);
                }
            }
        }

        public object Restore(ByteQue que)
        {
            if (que.Pop<bool>())
            {
                byte[] arr = new byte[que.PopSize()];
                for (int i = 0; i < arr.Length; ++i)
                {
                    arr[i] = que.Pop<byte>();
                }
                return Encoding.UTF8.GetString(arr);
            }
            return null;
        }
    }
}
