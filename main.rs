use rand::Rng;
use rand::seq::SliceRandom;
use std::collections::HashMap;
use std::io::{self, Write};
use std::time::{Duration, Instant};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

#[derive(Clone, Debug)]
struct Resource {
    name: String,
    amount: f64,
    production_rate: f64,
    consumption_rate: f64,
}

#[derive(Clone, Debug)]
struct Building {
    name: String,
    resource_production: HashMap<String, f64>,
    resource_consumption: HashMap<String, f64>,
    construction_cost: HashMap<String, f64>,
    construction_time: f64,
    quantity: u32,
}

#[derive(Clone, Debug)]
struct Research {
    name: String,
    description: String,
    cost: f64,
    completed: bool,
    effects: Vec<ResearchEffect>,
}

#[derive(Clone, Debug)]
enum ResearchEffect {
    ResourceProductionBoost(String, f64),
    NewBuilding(Building),
    PopulationGrowthBoost(f64),
    HappinessBoost(f64),
}

struct Colony {
    resources: HashMap<String, Resource>,
    buildings: HashMap<String, Building>,
    research: Vec<Research>,
    population: f64,
    happiness: f64,
    tech_points: f64,
    day: u32,
    events: Vec<String>,
}

impl Colony {
    fn new() -> Self {
        let mut resources = HashMap::new();
        resources.insert("Food".to_string(), Resource { name: "Food".to_string(), amount: 1000.0, production_rate: 10.0, consumption_rate: 1.0 });
        resources.insert("Energy".to_string(), Resource { name: "Energy".to_string(), amount: 500.0, production_rate: 5.0, consumption_rate: 1.0 });
        resources.insert("Minerals".to_string(), Resource { name: "Minerals".to_string(), amount: 200.0, production_rate: 2.0, consumption_rate: 0.0 });
        resources.insert("Water".to_string(), Resource { name: "Water".to_string(), amount: 800.0, production_rate: 8.0, consumption_rate: 2.0 });

        let mut buildings = HashMap::new();
        buildings.insert("Farm".to_string(), Building {
            name: "Farm".to_string(),
            resource_production: HashMap::from([("Food".to_string(), 5.0)]),
            resource_consumption: HashMap::from([("Energy".to_string(), 1.0), ("Water".to_string(), 2.0)]),
            construction_cost: HashMap::from([("Minerals".to_string(), 50.0)]),
            construction_time: 5.0,
            quantity: 1,
        });
        buildings.insert("Solar Panel".to_string(), Building {
            name: "Solar Panel".to_string(),
            resource_production: HashMap::from([("Energy".to_string(), 3.0)]),
            resource_consumption: HashMap::new(),
            construction_cost: HashMap::from([("Minerals".to_string(), 30.0)]),
            construction_time: 3.0,
            quantity: 1,
        });
        buildings.insert("Water Extractor".to_string(), Building {
            name: "Water Extractor".to_string(),
            resource_production: HashMap::from([("Water".to_string(), 4.0)]),
            resource_consumption: HashMap::from([("Energy".to_string(), 2.0)]),
            construction_cost: HashMap::from([("Minerals".to_string(), 40.0)]),
            construction_time: 4.0,
            quantity: 1,
        });

        let research = vec![
            Research {
                name: "Advanced Farming".to_string(),
                description: "Increases food production by 50%".to_string(),
                cost: 100.0,
                completed: false,
                effects: vec![ResearchEffect::ResourceProductionBoost("Food".to_string(), 1.5)],
            },
            Research {
                name: "Efficient Solar Cells".to_string(),
                description: "Increases energy production by 50%".to_string(),
                cost: 150.0,
                completed: false,
                effects: vec![ResearchEffect::ResourceProductionBoost("Energy".to_string(), 1.5)],
            },
            Research {
                name: "Hydroponics".to_string(),
                description: "Unlocks Hydroponics Lab for food production".to_string(),
                cost: 200.0,
                completed: false,
                effects: vec![ResearchEffect::NewBuilding(Building {
                    name: "Hydroponics Lab".to_string(),
                    resource_production: HashMap::from([("Food".to_string(), 10.0)]),
                    resource_consumption: HashMap::from([("Energy".to_string(), 3.0), ("Water".to_string(), 5.0)]),
                    construction_cost: HashMap::from([("Minerals".to_string(), 100.0)]),
                    construction_time: 10.0,
                    quantity: 0,
                })],
            },
            Research {
                name: "Community Center".to_string(),
                description: "Boosts happiness and population growth".to_string(),
                cost: 250.0,
                completed: false,
                effects: vec![
                    ResearchEffect::HappinessBoost(10.0),
                    ResearchEffect::PopulationGrowthBoost(0.05),
                ],
            },
        ];

        Colony {
            resources,
            buildings,
            research,
            population: 100.0,
            happiness: 100.0,
            tech_points: 0.0,
            day: 1,
            events: Vec::new(),
        }
    }

    fn update(&mut self) {
        // Update resource amounts
        for (_, resource) in self.resources.iter_mut() {
            resource.amount += resource.production_rate - resource.consumption_rate;
            resource.amount = resource.amount.max(0.0);
        }

        // Population consumes food and water
        let food_amount = self.resources.get("Food").unwrap().amount;
        let water_amount = self.resources.get("Water").unwrap().amount;
        let food_consumed = self.population.min(food_amount);
        let water_consumed = self.population.min(water_amount);

        if let Some(food) = self.resources.get_mut("Food") {
            food.amount -= food_consumed;
        }
        if let Some(water) = self.resources.get_mut("Water") {
            water.amount -= water_consumed;
        }

        // Update population
        let growth_rate = if food_consumed >= self.population && water_consumed >= self.population { 0.01 } else { -0.01 };
        self.population += self.population * growth_rate;
        self.population = self.population.max(0.0);

        // Update happiness
        let food_ratio = food_consumed / self.population.max(1.0);
        let water_ratio = water_consumed / self.population.max(1.0);
        self.happiness += (food_ratio + water_ratio - 1.0) * 5.0;
        self.happiness = self.happiness.clamp(0.0, 100.0);

        // Generate tech points
        self.tech_points += 0.1 * self.population * (self.happiness / 100.0);

        self.day += 1;
    }

    fn build(&mut self, building_name: &str) -> Result<(), String> {
        let building = self.buildings.get_mut(building_name).ok_or("Building not found")?;
        
        // Check if we have enough resources to build
        for (resource, cost) in &building.construction_cost {
            let available = self.resources.get(resource).map(|r| r.amount).unwrap_or(0.0);
            if available < *cost {
                return Err(format!("Not enough {}.", resource));
            }
        }

        // Deduct construction costs
        for (resource, cost) in &building.construction_cost {
            if let Some(r) = self.resources.get_mut(resource) {
                r.amount -= cost;
            }
        }

        // Increase building quantity and update resource rates
        building.quantity += 1;
        for (resource, rate) in &building.resource_production {
            if let Some(r) = self.resources.get_mut(resource) {
                r.production_rate += rate;
            }
        }
        for (resource, rate) in &building.resource_consumption {
            if let Some(r) = self.resources.get_mut(resource) {
                r.consumption_rate += rate;
            }
        }

        Ok(())
    }

    fn research(&mut self, research_name: &str) -> Result<(), String> {
        let research = self.research.iter_mut().find(|r| r.name == research_name).ok_or("Research not found")?;
        
        if research.completed {
            return Err("Research already completed".to_string());
        }

        if self.tech_points < research.cost {
            return Err("Not enough tech points".to_string());
        }

        self.tech_points -= research.cost;
        research.completed = true;

        for effect in &research.effects {
            match effect {
                ResearchEffect::ResourceProductionBoost(resource, multiplier) => {
                    if let Some(r) = self.resources.get_mut(resource) {
                        r.production_rate *= multiplier;
                    }
                },
                ResearchEffect::NewBuilding(building) => {
                    self.buildings.insert(building.name.clone(), building.clone());
                },
                ResearchEffect::PopulationGrowthBoost(boost) => {
                    self.population *= 1.0 + boost;
                },
                ResearchEffect::HappinessBoost(boost) => {
                    self.happiness += boost;
                    self.happiness = self.happiness.min(100.0);
                },
            }
        }

        Ok(())
    }
}

enum InputMode {
    Normal,
    Build(String),
    Research(String),
}

fn main() -> io::Result<()> {
    let mut colony = Colony::new();
    let mut rng = rand::thread_rng();

    let mut stdout = io::stdout().into_raw_mode()?;
    let stdin = io::stdin();
    let mut keys = stdin.keys();

    write!(stdout, "{}{}", termion::clear::All, termion::cursor::Hide)?;
    stdout.flush()?;

    let mut last_update = Instant::now();
    let mut input_mode = InputMode::Normal;

    loop {
        if last_update.elapsed() >= Duration::from_millis(500) {
            colony.update();
            last_update = Instant::now();
        }

        draw_ui(&colony, &mut stdout, &input_mode)?;

        if let Some(Ok(key)) = keys.next() {
            match input_mode {
                InputMode::Normal => match key {
                    Key::Char('q') => break,
                    Key::Char('b') => input_mode = InputMode::Build(String::new()),
                    Key::Char('r') => input_mode = InputMode::Research(String::new()),
                    _ => {}
                },
                InputMode::Build(ref mut input) => {
                    handle_input(key, input, &mut input_mode, |name| {
                        match colony.build(name) {
                            Ok(_) => colony.events.push(format!("Building {} constructed successfully!", name)),
                            Err(e) => colony.events.push(format!("Error: {}", e)),
                        }
                    });
                },
                InputMode::Research(ref mut input) => {
                    handle_input(key, input, &mut input_mode, |name| {
                        match colony.research(name) {
                            Ok(_) => colony.events.push(format!("Research {} completed successfully!", name)),
                            Err(e) => colony.events.push(format!("Error: {}", e)),
                        }
                    });
                },
            }
        }

        if rng.gen_bool(0.02) {
            let event = random_event(&mut colony, &mut rng);
            colony.events.push(event);
        }

        stdout.flush()?;
    }

    write!(stdout, "{}", termion::cursor::Show)?;
    Ok(())
}

fn handle_input<F>(key: Key, input: &mut String, input_mode: &mut InputMode, action: F)
where
    F: FnOnce(&str),
{
    match key {
        Key::Char('\n') => {
            let name = input.trim();
            if !name.is_empty() {
                action(name);
            }
            *input_mode = InputMode::Normal;
        }
        Key::Char(c) => input.push(c),
        Key::Backspace => { input.pop(); }
        Key::Esc => *input_mode = InputMode::Normal,
        _ => {}
    }
}

fn draw_ui(colony: &Colony, stdout: &mut termion::raw::RawTerminal<io::Stdout>, input_mode: &InputMode) -> io::Result<()> {
    write!(stdout, "{}", termion::clear::All)?;
    write!(stdout, "{}Stellar Reality - Day {}", termion::cursor::Goto(1, 1), colony.day)?;
    write!(stdout, "{}Population: {:.0} | Happiness: {:.1}%", termion::cursor::Goto(1, 2), colony.population, colony.happiness)?;
    write!(stdout, "{}Tech Points: {:.1}", termion::cursor::Goto(1, 3), colony.tech_points)?;

    write!(stdout, "{}Resources:", termion::cursor::Goto(1, 5))?;
    for (i, (_, resource)) in colony.resources.iter().enumerate() {
        write!(stdout, "{}{}: {:.1} (+{:.1}/-{:.1})", 
               termion::cursor::Goto(1, 6 + i as u16),
               resource.name, resource.amount, resource.production_rate, resource.consumption_rate)?;
    }

    write!(stdout, "{}Buildings:", termion::cursor::Goto(40, 5))?;
    for (i, (_, building)) in colony.buildings.iter().enumerate() {
        write!(stdout, "{}{}: {}", 
               termion::cursor::Goto(40, 6 + i as u16),
               building.name, building.quantity)?;
    }

    write!(stdout, "{}Research:", termion::cursor::Goto(1, 15))?;
    for (i, research) in colony.research.iter().enumerate() {
        write!(stdout, "{}{}: {} - {}", 
               termion::cursor::Goto(1, 16 + i as u16),
               research.name, 
               if research.completed { "Completed" } else { "Not Started" },
               research.description)?;
    }

    write!(stdout, "{}Recent Events:", termion::cursor::Goto(1, 22))?;
    for (i, event) in colony.events.iter().rev().take(3).enumerate() {
        write!(stdout, "{}{}", termion::cursor::Goto(1, 23 + i as u16), event)?;
    }

    match input_mode {
        InputMode::Normal => {
            write!(stdout, "{}Commands: (b) Build, (r) Research, (q) Quit", termion::cursor::Goto(1, 27))?;
        }
        InputMode::Build(input) => {
            write!(stdout, "{}Enter building name to construct: {}", termion::cursor::Goto(1, 27), input)?;
        }
        InputMode::Research(input) => {
            write!(stdout, "{}Enter research name: {}", termion::cursor::Goto(1, 27), input)?;
        }
    }

    Ok(())
}

fn random_event(colony: &mut Colony, rng: &mut rand::rngs::ThreadRng) -> String {
    let events: [(&str, Box<dyn Fn(&mut Colony) -> &str>); 7] = [
        ("Solar Flare", Box::new(|c: &mut Colony| {
            c.resources.get_mut("Energy").unwrap().amount *= 1.5;
            "A solar flare has increased energy production!"
        })),
        ("Meteor Strike", Box::new(|c: &mut Colony| {
            c.resources.get_mut("Minerals").unwrap().amount += 100.0;
            "A meteor strike has yielded additional minerals!"
        })),
        ("Disease Outbreak", Box::new(|c: &mut Colony| {
            c.population *= 0.9;
            c.happiness -= 10.0;
            "A disease outbreak has reduced the population and happiness."
        })),
        ("Technological Breakthrough", Box::new(|c: &mut Colony| {
            c.tech_points += 50.0;
            "A technological breakthrough has yielded additional tech points!"
        })),
        ("Alien Artifact Discovered", Box::new(|c: &mut Colony| {
            c.tech_points += 100.0;
            c.happiness += 5.0;
            "An alien artifact has been discovered, boosting research and morale!"
        })),
        ("Water Source Found", Box::new(|c: &mut Colony| {
            c.resources.get_mut("Water").unwrap().amount += 200.0;
            "A new water source has been discovered!"
        })),
        ("Volunteer Program", Box::new(|c: &mut Colony| {
            c.happiness += 15.0;
            c.population *= 1.05;
            "A successful volunteer program has boosted happiness and attracted new colonists!"
        })),
    ];

    let (name, event) = events.choose(rng).unwrap();
    let result = event(colony);
    format!("{} - {}", name, result)
}