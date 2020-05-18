package com.lrpc.tcp;

import java.io.IOException;
import java.net.ServerSocket;
import java.net.Socket;
import java.util.ArrayList;
import com.lrpc.buf.RecvBuf;
import com.lrpc.buf.SendData;
import com.lrpc.fun.Fun;

/**
 * synchronous TCP receiving data calling registration method
 */
public class Service implements java.io.Closeable {

    private Fun srvFun;
    private ArrayList<ServerSocket> srvs;

    /**
     * @param fun call the function with Fun
     */
    public Service(Fun fun) {
        srvFun = fun;
        srvs = new ArrayList<>();
    }

    /**
     * close all ServerSockets
     */
    public void close() {
        for (ServerSocket srv : srvs) {
            try {
                srv.close();
            } catch (IOException _e) {
            }
        }
    }

    /**
     * this method is blocked for running services
     * 
     * @param port listening port
     */
    public void run(int port) throws IOException {
        ServerSocket srv = new ServerSocket(port);
        srv.setReuseAddress(true);
        srvs.add(srv);
        while (true) {
            Socket socket = srv.accept();
            new Thread(() -> {
                try {
                    byte[] buf = new byte[1024];
                    while (true) {
                        RecvBuf recv = new RecvBuf();
                        while (true) {
                            if (recv.size() != null && recv.size() == recv.len()) {
                                break;
                            }
                            int read = socket.getInputStream().read(buf);
                            if (read > 0) {
                                recv.append(buf, read);
                            } else {
                                socket.shutdownInput();
                                socket.shutdownOutput();
                                socket.close();
                                return;
                            }
                        }
                        socket.getOutputStream().write(new SendData(srvFun.invoke(recv.byteQue())).toArray());
                    }
                } catch (Exception _e) {
                    try {
                        socket.shutdownInput();
                        socket.shutdownOutput();
                        socket.close();
                    } catch (Exception __e) {
                    }
                }
            }).start();
        }
    }
}