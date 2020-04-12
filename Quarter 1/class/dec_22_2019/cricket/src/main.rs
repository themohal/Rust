#[derive(Debug)]

struct Team {
    country:String,
    score : u16,
}
impl Team {
    fn print(&self)  {
        println!("{:#?}",self.score)
    }
    fn high (&self , other:&Team) ->u16  {
        if self.score > other.score {
            self.score
        }
        else {
        other.score
        }
    }
    fn ret_high_inst(self, newinstance:Team)-> Team { //fo this you need to move owner ship
        if self.score > newinstance.score {
            self
        }
        else {
            newinstance
        }
    }

}

fn main() {

    let team1 = Team {
        country : "Pakistan".to_string(),
        score : 435,
    };
    let team2= Team {
        country : "SriLinka".to_string(),
        score : 271, 
    };
        team1.print();
        team2.print();
        //let today_high = team1.high();
    let today_high = team1.high(&team2);
    println!("in main {}",today_high );
    let ret= team1.ret_high_inst(team2);
    println!("Printed :{:#?}",ret );
    }
