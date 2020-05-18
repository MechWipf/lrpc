package com.lrpc.fun;

import java.lang.reflect.InvocationTargetException;
import java.lang.reflect.Method;
import java.lang.reflect.Parameter;
import java.nio.charset.Charset;
import java.util.HashMap;
import com.lrpc.val.ByteQue;

/**
 * register call method
 */
public class Fun {

    private class ObjectMethod {
        Object obj;
        Method fun;
    }

    private HashMap<String, ObjectMethod> funs = new HashMap<>();

    /**
     * register the method being called
     * 
     * @param name the name to use when calling
     * @param obj  method object
     * @param fun  the actual method used
     */
    public void regist(String name, Object obj, Method fun) {
        ObjectMethod om = new ObjectMethod();
        om.obj = obj;
        om.fun = fun;
        funs.put(name, om);
    }

    /**
     * register the called method through annotations
     * 
     * @param obj method object
     */
    public void regist(Object obj) {
        Method[] funs = obj.getClass().getDeclaredMethods();
        for (Method fun : funs) {
            if (fun.isAnnotationPresent(Regist.class)) {
                regist(fun.getAnnotation(Regist.class).value(), obj, fun);
            }
        }
    }

    private ByteQue exception(String msg) {
        ByteQue que = new ByteQue();
        que.push(String.class, msg);
        return que;
    }

    /**
     * call the registered method
     * 
     * @param que the queue generated by fun
     * @return serialize error messages or call results into a queue
     */
    public ByteQue invoke(ByteQue que) {
        byte[] arr = new byte[que.popSize()];
        for (int i = 0; i < arr.length; ++i) {
            arr[i] = (byte) que.pop(byte.class);
        }
        String name = new String(arr, Charset.forName("UTF-8"));
        ObjectMethod om = funs.get(name);
        if (om == null) {
            return exception(name + " function not found");
        }
        Parameter[] types = om.fun.getParameters();
        Object[] args = new Object[types.length];
        for (int i = 0; i < types.length; ++i) {
            Class<?> type = types[i].getType();
            if (que.len() == 0) {
                return exception("error when calling function " + name + " to restore parameters to the " + i
                        + "th parameter " + type.toString());
            }
            try {
                args[i] = que.pop(type);
            } catch (UnsupportedOperationException e) {
                return exception("error when calling function " + name + " to restore parameters to the " + i
                        + "th parameter " + type.toString() + ": " + e.toString());
            }
        }
        if (que.len() != 0) {
            return exception("error when calling function " + name + " to restore parameters");
        }
        ByteQue ret = new ByteQue();
        om.fun.setAccessible(true);
        Object rst;
        try {
            rst = om.fun.invoke(om.obj, args);
        } catch (IllegalAccessException | IllegalArgumentException | InvocationTargetException e) {
            return exception("error calling function " + name + " " + e.toString());
        }
        ret.push(boolean.class, false);
        Class<?> rtp = om.fun.getReturnType();
        if (rtp != Void.TYPE) {
            try {
                ret.push(rtp, rst);
            } catch (UnsupportedOperationException e) {
                return exception("error calling function " + name + " to store result " + e.toString());
            }
        }
        return ret;
    }

    /**
     * call the method serialized into a queue
     * 
     * @param name  the name of the calling method
     * @param wraps whether the calling method wraps parameters, if Integer is true,
     *              int is false
     * @param args  calling method parameters
     * @return the result of calling the method
     * @throws UnsupportedOperationException exception during parameter reflection
     */
    public static ByteQue make(String name, boolean wraps, Object... args) throws UnsupportedOperationException {
        ByteQue que = new ByteQue();
        byte[] arr = name.getBytes(Charset.forName("UTF-8"));
        que.pushSize(arr.length);
        for (byte ch : arr) {
            que.push(byte.class, ch);
        }
        if (wraps) {
            for (Object arg : args) {
                que.push(arg.getClass(), arg);
            }
        } else {
            for (Object arg : args) {
                Class<?> type = arg.getClass();
                if (type == Byte.class) {
                    que.push(byte.class, arg);
                } else if (type == Short.class) {
                    que.push(short.class, arg);
                } else if (type == Integer.class) {
                    que.push(int.class, arg);
                } else if (type == Long.class) {
                    que.push(long.class, arg);
                } else if (type == Float.class) {
                    que.push(float.class, arg);
                } else if (type == Double.class) {
                    que.push(double.class, arg);
                } else if (type == Boolean.class) {
                    que.push(boolean.class, arg);
                } else if (type == Character.class) {
                    que.push(char.class, arg);
                } else {
                    que.push(type, arg);
                }
            }
        }
        return que;
    }
}