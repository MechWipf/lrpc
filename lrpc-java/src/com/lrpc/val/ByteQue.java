package com.lrpc.val;

import java.lang.reflect.Array;
import java.lang.reflect.Constructor;
import java.lang.reflect.Field;
import java.lang.reflect.InvocationTargetException;
import java.util.HashMap;

/**
 * store and restore data queue
 */
public class ByteQue {

    private static final HashMap<Class<?>, Store> STORES = new HashMap<>();
    static {
        STORES.put(Byte.class, new ByteStore());
        STORES.put(Short.class, new ShortStore());
        STORES.put(Integer.class, new IntegerStore());
        STORES.put(Long.class, new LongStore());
        STORES.put(Float.class, new FloatStore());
        STORES.put(Double.class, new DoubleStore());
        STORES.put(Boolean.class, new BooleanStore());
        STORES.put(Character.class, new CharacterStore());
        STORES.put(String.class, new StringStore());
    }

    /**
     * register a custom data store
     * 
     * @param type  store and restore data types
     * @param store store and restore methods
     */
    public static void regist(Class<?> type, Store store) {
        STORES.put(type, store);
    }

    private byte[] buff = new byte[16];
    private int head = 0;
    private int tail = 0;

    private void push(byte val) {
        buff[tail] = val;
        if ((tail = tail + 1 & buff.length - 1) == head) {
            byte[] a = new byte[buff.length << 1];
            System.arraycopy(buff, head, a, 0, buff.length - head);
            System.arraycopy(buff, 0, a, buff.length - head, head);
            head = 0;
            tail = buff.length;
            buff = a;
        }
    }

    private byte pop() {
        if (head == tail) {
            return 0;
        }
        byte v = buff[head];
        head = head + 1 & buff.length - 1;
        return v;
    }

    /**
     * length
     */
    public int len() {
        return tail - head & buff.length - 1;
    }

    /**
     * the specified part of the array is added to the end of the queue
     * 
     * @param arr array containing data
     * @param six starting index
     * @param eix end index, excluding the element at this position
     */
    public void addAll(byte[] arr, int six, int eix) {
        if (six < 0) {
            six = 0;
        }
        if (eix > arr.length || eix < 0) {
            eix = arr.length;
        }
        if (six < eix) {
            if (buff.length - len() <= eix - six) {
                int len = buff.length;
                while (len <= eix - six + len()) {
                    len <<= 1;
                }
                byte[] a = new byte[len];
                if (head < tail) {
                    tail = len();
                    System.arraycopy(buff, head, a, 0, tail);
                } else if (head > tail) {
                    System.arraycopy(buff, head, a, 0, buff.length - head);
                    System.arraycopy(buff, 0, a, buff.length - head, tail);
                    tail = len();
                } else {
                    tail = 0;
                }
                System.arraycopy(arr, six, a, tail, eix - six);
                head = 0;
                tail += eix - six;
                buff = a;
            } else {
                if (tail < head || eix - six <= buff.length - tail) {
                    System.arraycopy(arr, six, buff, tail, eix - six);
                    tail += eix - six;
                } else {
                    System.arraycopy(arr, six, buff, tail, buff.length - tail);
                    System.arraycopy(arr, buff.length - tail, buff, 0, eix - six - buff.length + tail);
                    tail += eix - six - buff.length;
                }
            }
        }
    }

    /**
     * convert queue to byte array
     */
    public byte[] toArray() {
        byte[] a = new byte[len()];
        if (head < tail) {
            System.arraycopy(buff, head, a, 0, a.length);
        } else if (head > tail) {
            System.arraycopy(buff, head, a, 0, buff.length - head);
            System.arraycopy(buff, 0, a, buff.length - head, tail);
        }
        return a;
    }

    /**
     * saved data length is not used for the value
     */
    public void pushSize(int val) {
        for (int i = 0; i < 5; ++i) {
            if (val <= 0x7f && val >= 0) {
                push((byte) (val & 0x7f));
                break;
            } else {
                push((byte) (val & 0x7f | 0x80));
            }
            val >>= 7;
        }
    }

    /**
     * restore data length is not used for the value
     */
    public int popSize() {
        int s = 0;
        for (int i = 0; i < 5; ++i) {
            byte v = pop();
            s |= (v & 0x7f) << 7 * i;
            if (v >= 0) {
                break;
            }
        }
        return s;
    }

    /**
     * to save data to the queue, you need to have a parameterless constructor when
     * using reflection
     * 
     * @param type type of data
     * @param val  data value
     * @throws UnsupportedOperationException all other exceptions are turned into
     *                                       runtime
     */
    public void push(Class<?> type, Object val) throws UnsupportedOperationException {
        if (type == byte.class) {
            push((byte) val);
        } else if (type == short.class) {
            short v = (short) val;
            push((byte) v);
            push((byte) (v >> 8));
        } else if (type == int.class) {
            int v = (int) val;
            push((byte) v);
            push((byte) (v >> 8));
            push((byte) (v >> 16));
            push((byte) (v >> 24));
        } else if (type == long.class) {
            long v = (long) val;
            push((byte) v);
            push((byte) (v >> 8));
            push((byte) (v >> 16));
            push((byte) (v >> 24));
            push((byte) (v >> 32));
            push((byte) (v >> 40));
            push((byte) (v >> 48));
            push((byte) (v >> 56));
        } else if (type == float.class) {
            int v = Float.floatToIntBits((float) val);
            push((byte) v);
            push((byte) (v >> 8));
            push((byte) (v >> 16));
            push((byte) (v >> 24));
        } else if (type == double.class) {
            long v = Double.doubleToLongBits((double) val);
            push((byte) v);
            push((byte) (v >> 8));
            push((byte) (v >> 16));
            push((byte) (v >> 24));
            push((byte) (v >> 32));
            push((byte) (v >> 40));
            push((byte) (v >> 48));
            push((byte) (v >> 56));
        } else if (type == boolean.class) {
            boolean v = (boolean) val;
            push((byte) (v ? 1 : 0));
        } else if (type == char.class) {
            char v = (char) val;
            push((byte) v);
            push((byte) (v >> 8));
        } else if (STORES.containsKey(type)) {
            STORES.get(type).store(this, val);
        } else if (type.isArray()) {
            if (val == null) {
                pushSize(0);
            } else {
                int len = Array.getLength(val);
                pushSize(len);
                Class<?> cot = type.getComponentType();
                for (int i = 0; i < len; ++i) {
                    push(cot, Array.get(val, i));
                }
            }
        } else if (type.isAnnotationPresent(StoreFields.class)) {
            StoreFields anno = type.getAnnotation(StoreFields.class);
            String[] fns = anno.value().split(",");
            for (String fn : fns) {
                Field fld;
                try {
                    fld = type.getDeclaredField(fn);
                } catch (NoSuchFieldException | SecurityException _e) {
                    try {
                        fld = type.getField(fn);
                    } catch (NoSuchFieldException | SecurityException e) {
                        throw new UnsupportedOperationException(e);
                    }
                }
                fld.setAccessible(true);
                Object obj;
                try {
                    obj = fld.get(val);
                } catch (IllegalArgumentException | IllegalAccessException e) {
                    throw new UnsupportedOperationException(e);
                }
                push(fld.getType(), obj);
            }
        } else {
            Field[] flds = type.getDeclaredFields();
            for (Field fld : flds) {
                fld.setAccessible(true);
                Object obj;
                try {
                    obj = fld.get(val);
                } catch (IllegalArgumentException | IllegalAccessException e) {
                    throw new UnsupportedOperationException(e);
                }
                push(fld.getType(), obj);
            }
        }
    }

    /**
     * to restore data from the queue, you need a parameterless constructor when
     * using reflection
     * 
     * @param type type of data to be restored
     * @throws UnsupportedOperationException all other exceptions are turned into
     *                                       runtime
     */
    public Object pop(Class<?> type) throws UnsupportedOperationException {
        if (type == byte.class) {
            return pop();
        }
        if (type == short.class) {
            return (short) (pop() & 0xff | pop() << 8);
        }
        if (type == int.class) {
            return pop() & 0xff | (pop() & 0xff) << 8 | (pop() & 0xff) << 16 | pop() << 24;
        }
        if (type == long.class) {
            return (long) (pop() & 0xff) | (long) (pop() & 0xff) << 8 | (long) (pop() & 0xff) << 16
                    | (long) (pop() & 0xff) << 24 | (long) (pop() & 0xff) << 32 | (long) (pop() & 0xff) << 40
                    | (long) (pop() & 0xff) << 48 | (long) (pop() & 0xff) << 56;
        }
        if (type == float.class) {
            return Float.intBitsToFloat(pop() & 0xff | (pop() & 0xff) << 8 | (pop() & 0xff) << 16 | pop() << 24);
        }
        if (type == double.class) {
            return Double.longBitsToDouble((long) (pop() & 0xff) | (long) (pop() & 0xff) << 8
                    | (long) (pop() & 0xff) << 16 | (long) (pop() & 0xff) << 24 | (long) (pop() & 0xff) << 32
                    | (long) (pop() & 0xff) << 40 | (long) (pop() & 0xff) << 48 | (long) (pop() & 0xff) << 56);
        }
        if (type == boolean.class) {
            return pop() != 0;
        }
        if (type == char.class) {
            return (char) (pop() & 0xff | pop() << 8);
        }
        if (STORES.containsKey(type)) {
            return STORES.get(type).restore(this);
        }
        if (type.isArray()) {
            int len = popSize();
            Class<?> cot = type.getComponentType();
            Object obj = Array.newInstance(cot, len);
            for (int i = 0; i < len; ++i) {
                Array.set(obj, i, pop(cot));
            }
            return obj;
        }
        Constructor<?> cons;
        try {
            cons = type.getDeclaredConstructor();
        } catch (NoSuchMethodException | SecurityException e) {
            throw new UnsupportedOperationException(e);
        }
        cons.setAccessible(true);
        Object obj;
        try {
            obj = cons.newInstance();
        } catch (InstantiationException | IllegalAccessException | IllegalArgumentException
                | InvocationTargetException e) {
            throw new UnsupportedOperationException(e);
        }
        if (type.isAnnotationPresent(StoreFields.class)) {
            StoreFields anno = type.getAnnotation(StoreFields.class);
            String[] fns = anno.value().split(",");
            for (String fn : fns) {
                Field fld;
                try {
                    fld = type.getDeclaredField(fn);
                } catch (NoSuchFieldException | SecurityException _e) {
                    try {
                        fld = type.getField(fn);
                    } catch (NoSuchFieldException | SecurityException e) {
                        throw new UnsupportedOperationException(e);
                    }
                }
                fld.setAccessible(true);
                Object val = pop(fld.getType());
                try {
                    fld.set(obj, val);
                } catch (IllegalArgumentException | IllegalAccessException e) {
                    throw new UnsupportedOperationException(e);
                }
            }
        } else {
            Field[] flds = type.getDeclaredFields();
            for (Field fld : flds) {
                fld.setAccessible(true);
                Object val = pop(fld.getType());
                try {
                    fld.set(obj, val);
                } catch (IllegalArgumentException | IllegalAccessException e) {
                    throw new UnsupportedOperationException(e);
                }
            }
        }
        return obj;
    }
}