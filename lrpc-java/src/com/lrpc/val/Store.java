package com.lrpc.val;

import java.nio.charset.Charset;

/**
 * rules for storing and restoring data
 */
public interface Store {

    /**
     * storing data
     * 
     * @param que queue for storing data
     * @param val the data to be stored is not a class that implements this
     *            interface
     */
    void store(ByteQue que, Object val);

    /**
     * restoring data
     * 
     * @param que queue for restoring data
     * @return previously stored data
     */
    Object restore(ByteQue que);
}

class ByteStore implements Store {

    @Override
    public void store(ByteQue que, Object val) {
        if (val == null) {
            que.push(boolean.class, false);
        } else {
            que.push(boolean.class, true);
            que.push(byte.class, val);
        }
    }

    @Override
    public Object restore(ByteQue que) {
        if ((boolean) que.pop(boolean.class)) {
            return que.pop(byte.class);
        }
        return null;
    }
}

class ShortStore implements Store {

    @Override
    public void store(ByteQue que, Object val) {
        if (val == null) {
            que.push(boolean.class, false);
        } else {
            que.push(boolean.class, true);
            que.push(short.class, val);
        }
    }

    @Override
    public Object restore(ByteQue que) {
        if ((boolean) que.pop(boolean.class)) {
            return que.pop(short.class);
        }
        return null;
    }
}

class IntegerStore implements Store {

    @Override
    public void store(ByteQue que, Object val) {
        if (val == null) {
            que.push(boolean.class, false);
        } else {
            que.push(boolean.class, true);
            que.push(int.class, val);
        }
    }

    @Override
    public Object restore(ByteQue que) {
        if ((boolean) que.pop(boolean.class)) {
            return que.pop(int.class);
        }
        return null;
    }
}

class LongStore implements Store {

    @Override
    public void store(ByteQue que, Object val) {
        if (val == null) {
            que.push(boolean.class, false);
        } else {
            que.push(boolean.class, true);
            que.push(long.class, val);
        }
    }

    @Override
    public Object restore(ByteQue que) {
        if ((boolean) que.pop(boolean.class)) {
            return que.pop(long.class);
        }
        return null;
    }
}

class FloatStore implements Store {

    @Override
    public void store(ByteQue que, Object val) {
        if (val == null) {
            que.push(boolean.class, false);
        } else {
            que.push(boolean.class, true);
            que.push(float.class, val);
        }
    }

    @Override
    public Object restore(ByteQue que) {
        if ((boolean) que.pop(boolean.class)) {
            return que.pop(float.class);
        }
        return null;
    }
}

class DoubleStore implements Store {

    @Override
    public void store(ByteQue que, Object val) {
        if (val == null) {
            que.push(boolean.class, false);
        } else {
            que.push(boolean.class, true);
            que.push(double.class, val);
        }
    }

    @Override
    public Object restore(ByteQue que) {
        if ((boolean) que.pop(boolean.class)) {
            return que.pop(double.class);
        }
        return null;
    }
}

class BooleanStore implements Store {

    @Override
    public void store(ByteQue que, Object val) {
        if (val == null) {
            que.push(boolean.class, false);
        } else {
            que.push(boolean.class, true);
            que.push(boolean.class, val);
        }
    }

    @Override
    public Object restore(ByteQue que) {
        if ((boolean) que.pop(boolean.class)) {
            return que.pop(boolean.class);
        }
        return null;
    }
}

class CharacterStore implements Store {

    @Override
    public void store(ByteQue que, Object val) {
        if (val == null) {
            que.push(boolean.class, false);
        } else {
            que.push(boolean.class, true);
            que.push(char.class, val);
        }
    }

    @Override
    public Object restore(ByteQue que) {
        if ((boolean) que.pop(boolean.class)) {
            return que.pop(char.class);
        }
        return null;
    }
}

class StringStore implements Store {

    @Override
    public void store(ByteQue que, Object val) {
        if (val == null) {
            que.push(boolean.class, false);
        } else {
            que.push(boolean.class, true);
            byte[] arr = ((String) val).getBytes(Charset.forName("UTF-8"));
            que.pushSize(arr.length);
            for (byte ch : arr) {
                que.push(byte.class, ch);
            }
        }
    }

    @Override
    public Object restore(ByteQue que) {
        if ((boolean) que.pop(boolean.class)) {
            byte[] arr = new byte[que.popSize()];
            for (int i = 0; i < arr.length; ++i) {
                arr[i] = (byte) que.pop(byte.class);
            }
            return new String(arr, Charset.forName("UTF-8"));
        }
        return null;
    }
}