# lrpc
rust go java c # use synchronous tcp to call each other, data types correspond to each other

*rust Examples*

```
use lrpc::*;

#[derive(CommonStore, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(CommonStore, Debug)]
struct Circle {
    center: Point,
    radius: i32,
}

#[fmt_function]
fn new_circle(p: Point, r: i32) -> Circle {
    Circle {
        center: p,
        radius: r,
    }
}

#[fmt_function]
fn circle_area(c: Circle) -> f64 {
    let f_radius = c.radius as f64;
    f_radius * f_radius * 3.14159
}

fn main() {
    let mut srv_fun = Fun::new();
    srv_fun.regist("new_circle", new_circle);
    srv_fun.regist("circle_area", circle_area);

    //start service
    std::thread::spawn(move || {
        service(srv_fun, "0.0.0.0:9009");
    });
    std::thread::sleep(std::time::Duration::from_millis(10));

    let mut conn = Connection::new("127.0.0.1:9009");
    let circle: Result<Circle> = conn.invoke(fun!("new_circle", Point { x: 400, y: 300 }, 100));
    if let Ok(circle) = circle {
        println!("{:?}", circle);
        let area: Result<f64> = conn.invoke(fun!("circle_area", circle));
        println!("{:?}", area);
    }
}
```

*go Examples*

```
package main

import (
	"fmt"
	"lrpc-go/fun"
	"lrpc-go/tcp"
	"lrpc-go/val"
	"reflect"
	"time"
)

type Point struct {
	X int32
	Y int32
}

type Circle struct {
	Center Point
	Radius int32
}

func CreatCircle(p Point, r int32) Circle {
	return Circle{
		Center: p,
		Radius: r,
	}
}

func Area(circle Circle) float64 {
	return float64(circle.Radius*circle.Radius) * 3.14159
}

func main() {
	val.RegistObj(Point{})
	val.RegistObj(Circle{})

	srvFun := fun.NewFun()
	srvFun.Regist("new_circle", CreatCircle)
	srvFun.Regist("circle_area", Area)

	go func() {
		e := tcp.Service(srvFun, "0.0.0.0:9009")
		fmt.Println(e)
	}()
	time.Sleep(time.Microsecond * 10)

	conn, e := tcp.NewConnection("127.0.0.1:9009")
	if e != nil {
		fmt.Println(e)
		return
	}
	fn, e := fun.Make("new_circle", Point{X: 400, Y: 300}, int32(100))
	if e != nil {
		fmt.Println(e)
		return
	}
	ra, e := conn.Invoke(fn, reflect.TypeOf(Circle{}).String())
	if e != nil {
		fmt.Println(e)
		return
	}
	circle := ra[0].(Circle)
	fmt.Println(circle)
	fn, e = fun.Make("circle_area", circle)
	if e != nil {
		fmt.Println(e)
		return
	}
	ra, e = conn.Invoke(fn, "float64")
	if e != nil {
		fmt.Println(e)
		return
	}
	area := ra[0].(float64)
	fmt.Println(area)
	conn.Close()
}
``` 

*java Examples*

```
import java.io.IOException;
import com.lrpc.fun.Fun;
import com.lrpc.fun.Regist;
import com.lrpc.tcp.Connection;
import com.lrpc.tcp.Service;
import com.lrpc.val.StoreFields;

@StoreFields("x,y")
class Point {
    int x;
    int y;
}

@StoreFields("center,radius")
class Circle {
    Point center;
    int radius;

    @Regist("new_circle")
    public Circle circle(Point p, int r) {
        center = p;
        radius = r;
        return this;
    }

    public static double area(Circle circle) {
        return circle.radius * circle.radius * 3.14159;
    }
}

public class Main {

    public static void main(String[] args) {
        try {
            Fun srvFun = new Fun();
            Circle srvCircle = new Circle();
            srvFun.regist(srvCircle);
            srvFun.regist("circle_area", null, Circle.class.getMethod("area", Circle.class));

            new Thread(() -> {
                try {
                    Service srv = new Service(srvFun);
                    srv.run(9009);
                    srv.close();
                } catch (IOException e) {
                    e.printStackTrace();
                }
            }).start();
            Thread.sleep(10);

            Connection conn = new Connection("127.0.0.1", 9009);
            Point point = new Point();
            point.x = 400;
            point.y = 300;
            Circle circle = (Circle) conn.invoke(Fun.make("new_circle", false, point, 100), Circle.class);
            System.out.println("x:" + circle.center.x + " y:" + circle.center.y + " r:" + circle.radius);
            double area = (double) conn.invoke(Fun.make("circle_area", false, circle), double.class);
            System.out.println(area);
            conn.close();
        } catch (Exception e) {
            e.printStackTrace();
        }
    }
}
``` 

*c# Examples*

```
using System;
using System.Threading;
using lrpc.val;
using lrpc.fun;
using lrpc.buf;
using lrpc.tcp;

namespace main
{
    class Point
    {
        public int x;
        public int y;
    }

    class Circle
    {
        public Point center;
        public int radius;

        public Circle CreatCircle(Point p, int r)
        {
            center = p;
            radius = r;
            return this;
        }

        public static double Area(Circle circle)
        {
            return circle.radius * circle.radius * 3.14159;
        }
    }

    class Program
    {
        static void Main(string[] args)
        {
            try
            {
                Fun srvFun = new Fun();
                Circle srvCircle = new Circle();
                srvFun.Regist("new_circle", srvCircle, srvCircle.GetType().GetMethod("CreatCircle"));
                srvFun.Regist("circle_area", null, typeof(Circle).GetMethod("Area"));

                new Thread(() =>
                {
                    Service srv = new Service(srvFun);
                    srv.Run(9009);
                    srv.Dispose();
                }).Start();
                Thread.Sleep(10);

                Connection conn = new Connection("127.0.0.1", 9009);
                Point point = new Point();
                point.x = 400;
                point.y = 300;
                Circle circle = conn.Invoke<Circle>(Fun.Make("new_circle", point, 100));
                Console.WriteLine("x:" + circle.center.x + " y:" + circle.center.y + " r:" + circle.radius);
                double area = conn.Invoke<double>(Fun.Make("circle_area", circle));
                Console.WriteLine(area);
                conn.Dispose();
            }
            catch (Exception e)
            {
                Console.WriteLine(e);
            }
        }
    }
}
``` 

## License

lrpc is provided under the MIT license. See [LICENSE](LICENSE).
