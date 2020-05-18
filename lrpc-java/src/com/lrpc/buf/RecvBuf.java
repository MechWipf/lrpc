package com.lrpc.buf;

import com.lrpc.val.ByteQue;

/**
 * get actual length data
 */
public class RecvBuf {

    private ByteQue buff = new ByteQue();
    private Integer size = null;

    /**
     * add some data to the buffer until it reaches the specified length
     * 
     * @param other  part of the data
     * @param length actual data length
     */
    public void append(byte[] other, int length) {
        if (length < 0) {
            length = other.length;
        }
        if (size == null) {
            if (buff.len() == 0) {
                for (int x = 0; x < length; ++x) {
                    if (x == 4 || other[x] >= 0) {
                        int s = 0;
                        for (int i = 0; i <= x; ++i) {
                            s |= (other[i] & 0x7f) << 7 * i;
                        }
                        size = s;
                        s += x + 1;
                        if (s < length) {
                            buff.addAll(other, x + 1, s);
                        } else {
                            buff.addAll(other, x + 1, length);
                        }
                        return;
                    }
                }
                buff.addAll(other, 0, length);
            } else {
                buff.addAll(other, 0, length);
                byte[] arr = buff.toArray();
                for (int x = 0; x < arr.length; ++x) {
                    if (x == 4 || arr[x] >= 0) {
                        int s = 0;
                        for (int i = 0; i <= x; ++i) {
                            s |= ((byte) buff.pop(byte.class) & 0x7f) << 7 * i;
                        }
                        size = s;
                        s += x + 1;
                        if (arr.length > s) {
                            buff = new ByteQue();
                            buff.addAll(arr, x + 1, s);
                        }
                        break;
                    }
                }
            }
        } else {
            if (size > buff.len()) {
                int l = size - buff.len();
                if (l < length) {
                    buff.addAll(other, 0, l);
                } else {
                    buff.addAll(other, 0, length);
                }
            }
        }
    }

    /**
     * the length of data that should be received
     */
    public Integer size() {
        return size;
    }

    /**
     * length of data received
     */
    public int len() {
        return buff.len();
    }

    /**
     * get ByteQue
     */
    public ByteQue byteQue() {
        return buff;
    }
}