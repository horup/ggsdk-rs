/*for i in 0..10 {
    print("1");
    spawn_ball();
    print("2");
}*/

fn start() {
    for i in 0..10 {
        let ball = spawn_ball();
        print("ball spawned at " + ball[0] + "," + ball[1]);
    }
}

fn update_ball(ball) {
    print(ball.pos);
}