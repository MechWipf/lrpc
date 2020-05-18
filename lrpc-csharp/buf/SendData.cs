using lrpc.val;

namespace lrpc.buf
{
    /// <summary>
    /// add length to the actual data to judge the integrity of the data
    /// </summary>
    public class SendData
    {
        private ByteQue buff;

        /// <param name="que">the actual data</param>
        public SendData(ByteQue que)
        {
            buff = que;
        }

        /// <summary>
        /// added length data
        /// </summary>
        public byte[] ToArray()
        {
            ByteQue que = new ByteQue();
            que.PushSize(buff.Len);
            que.AddAll(buff.ToArray(), 0, -1);
            return que.ToArray();
        }
    }
}
