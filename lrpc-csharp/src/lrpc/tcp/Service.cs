using System;
using System.Collections.Generic;
using System.Net.Sockets;
using System.Threading;
using System.Net;
using lrpc.fun;
using lrpc.buf;

namespace lrpc.tcp
{
    /// <summary>
    /// synchronous TCP receiving data calling registration method
    /// </summary>
    public class Service : IDisposable
    {
        private Fun srvFun;
        private List<Socket> srvs;

        /// <param name="fun">call the function with Fun</param>
        public Service(Fun fun)
        {
            srvFun = fun;
            srvs = new List<Socket>();
        }

        /// <summary>
        /// close all Sockets
        /// </summary>
        public void Dispose()
        {
            foreach (Socket srv in srvs)
            {
                srv.Dispose();
            }
        }

        /// <summary>
        /// this method is blocked for running services
        /// </summary>
        /// <param name="port">listening port</param>
        public void Run(int port)
        {
            Socket srv = new Socket(AddressFamily.InterNetwork, SocketType.Stream, ProtocolType.Tcp);
            srv.ExclusiveAddressUse = false;
            srv.Bind(new IPEndPoint(IPAddress.Any, port));
            srv.Listen(int.MaxValue);
            srvs.Add(srv);
            while (true)
            {
                Socket socket = srv.Accept();
                new Thread(() =>
                {
                    try
                    {
                        byte[] buf = new byte[1024];
                        while (true)
                        {
                            RecvBuf recv = new RecvBuf();
                            while (true)
                            {
                                if (recv.Size.HasValue && recv.Size.Value == recv.Len)
                                {
                                    break;
                                }
                                int read = socket.Receive(buf);
                                if (read > 0)
                                {
                                    recv.Append(buf, read);
                                }
                                else
                                {
                                    socket.Shutdown(SocketShutdown.Both);
                                    socket.Dispose();
                                    return;
                                }
                            }
                            byte[] rst = new SendData(srvFun.Invoke(recv.ByteQue)).ToArray();
                            int write = 0;
                            while (true)
                            {
                                write += socket.Send(rst, write, rst.Length - write, SocketFlags.None);
                                if (write >= rst.Length)
                                {
                                    break;
                                }
                            }
                        }
                    }
                    catch
                    {
                        socket.Shutdown(SocketShutdown.Both);
                        socket.Dispose();
                    }
                }).Start();
            }
        }
    }
}
