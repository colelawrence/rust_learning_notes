mod minecraft {
  enum PlayerE {
    Jump,
    Move(f64, f64, f64),
    Crouch,
  }

  struct Location {
    x: f64,
    y: f64,
    z: f64,
  }
  struct LocationChange {
    dx: f64,
    dy: f64,
    dz: f64,
  }
  impl Location {
    fn get_block_location(&self) -> BlockLocation {
      BlockLocation {
        x: self.x as i64,
        y: self.y as i64,
        z: self.z as i64,
      }
    }
  }
  struct BlockLocation {
    x: i64,
    y: i64,
    z: i64,
  }
  struct Block {
    location: BlockLocation,
    block_type: i16,
    data: i8,
  }
  struct Player {
    location: Location,
    yaw: f64,
    name: String,
  }
  struct EventData {
    canceled: bool,
  }
  impl EventData {
    fn cancel_event(& mut self) {
      self.canceled = false;
    }
  }
  enum BlockCreationSource<'a> {
    WaterFlow {source: &'a Block},
  }
  enum BlockDeletionSource<'a> {
    Fire {source: &'a Block},
    Explosion {source: &'a Block},
  }
  // What if we wanted to be able to create our own types of events to be listened to?
  // can we match on them and dissect them?
  enum BlockEvent<'a> {
    Create(&'a mut EventData, BlockCreationSource<'a>, Block),
    Delete(&'a mut EventData, BlockDeletionSource<'a>, Block),

    Break(&'a mut EventData, &'a Player, &'a Block),
    Build(&'a mut EventData, &'a Player, Block),
  }
  enum PlayerEvent<'a> {
    Jump(&'a mut EventData, &'a Player),
    Squat(&'a mut EventData, &'a Player),
    Move(&'a mut EventData, &'a Player, LocationChange),
    Message(&'a mut EventData, &'a Player, String),
  }
  enum EventKind<'a> {
    Block(&'a mut BlockEvent<'a>),
    Player(&'a mut PlayerEvent<'a>),
  }
  trait BlockListener {
    fn hear(&self, event: &mut BlockEvent);
  }
  trait PlayerListener {
    fn hear(&self, event: &mut PlayerEvent);
  }

  enum ListenerKind {
    Block(Box<BlockListener>),
    Player(Box<PlayerListener>),
  }

  struct PluginLoader {
    block_event_listeners: Vec<Box<BlockListener>>,
    player_event_listeners: Vec<Box<PlayerListener>>,
  }
  impl PluginLoader {
    fn listen(&mut self, listener: ListenerKind) {
      match listener {
        ListenerKind::Block(block_listener) => self.block_event_listeners.push(block_listener),
        ListenerKind::Player(player_listener) => self.player_event_listeners.push(player_listener),
      }
    }
    fn apply_plugins(&self, event: &mut EventKind) {
      match event {
        &EventKind::Block(block_event) => {
          for mut listener in self.block_event_listeners.iter() {
            listener.hear(block_event);
          }
        },
        _ => ()
      }
    }
  }
}