package com.lrpc.buf;

import com.lrpc.val.ByteQue;

/**
 * add length to the actual data to judge the integrity of the data
 */
public class SendData {

    private ByteQue buff;

    /**
     * @param que the actual data
     */
    public SendData(ByteQue que) {
        buff = que;
    }

    /**
     * added length data
     */
    public byte[] toArray() {
        ByteQue que = new ByteQue();
        que.pushSize(buff.len());
        que.addAll(buff.toArray(), 0, -1);
        return que.toArray();
    }
}