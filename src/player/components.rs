use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
#[require(Camera2d)]
pub struct PlayerCamera;

#[derive(Component)]
pub struct AnimationTimer(pub Timer);
#[derive(Component)]
pub struct AnimationFrame(pub u8);

#[derive(Component)]
pub struct PlayerMovementSpeed(pub f32);

#[derive(Component)]
pub struct PlayerAnimationTimer(pub Timer);

#[derive(Component)]
pub struct PlayerAnimationFrame(pub u8);

#[derive(Resource, Default)]
pub struct CurrentPlayerChunkPos(pub (i32, i32));

#[derive(Event)]
pub struct PlayerChunkUpdateEvent(pub (i32, i32));


#[derive(Component, Debug, PartialEq)]
pub struct PlayerHealth {
    current_health: u8,
    max_health: u8,
}

impl Default for PlayerHealth {
    fn default() -> Self {
        Self {
            current_health: 3,
            max_health: 3,
        }
    }
}

impl PlayerHealth {
    pub fn new(max_health: u8) -> Self {
        Self {
            current_health: max_health,
            max_health,
        }
    }
    pub fn increase_health(&mut self, amount: u8) {
        let new_health = self.current_health + amount;
        if new_health <= self.max_health {
            self.current_health = new_health
        } else {
            self.current_health = self.max_health
        }
    }
    pub fn decrease_health(&mut self, amount: u8) {
        if amount < self.current_health {
            self.current_health -= amount;
        } else {
            self.current_health = 0;
        }
    }
    pub fn increase_max_health(&mut self, amount: u8) {
        self.max_health += amount;
    }
    pub fn decrease_max_health(&mut self, amount: u8) {
        if amount <= self.max_health {
            self.max_health -= amount;
        } else {
            self.max_health = 0;
        }
        if self.max_health < self.current_health {
            self.current_health = self.max_health;
        }
    }

    pub fn current_health(&self) -> u8 {
        self.current_health
    }

    pub fn max_health(&self) -> u8 {
        self.max_health
    }
}

#[derive(Component)]
pub struct DamageInvulnerability {
    pub timer: Timer,
}

impl Default for DamageInvulnerability {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(1.0, TimerMode::Once)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_health_default() {
        let health = PlayerHealth::default();
        assert_eq!(
            health.current_health(),
            3,
            "Default current health should be 3"
        );
        assert_eq!(health.max_health(), 3, "Default max health should be 3");
    }

    #[test]
    fn test_player_health_new() {
        let health = PlayerHealth::new(10);
        assert_eq!(
            health.current_health(),
            10,
            "New current health should match max"
        );
        assert_eq!(health.max_health(), 10, "New max health should be set");

        let health_zero = PlayerHealth::new(0);
        assert_eq!(
            health_zero.current_health(),
            0,
            "New current health should match max (zero)"
        );
        assert_eq!(
            health_zero.max_health(),
            0,
            "New max health should be set (zero)"
        );
    }

    #[test]
    fn test_increase_health_normal() {
        let mut health = PlayerHealth {
            current_health: 5,
            max_health: 10,
        };
        health.increase_health(3);
        assert_eq!(
            health.current_health(),
            8,
            "Health should increase normally"
        );
        assert_eq!(
            health.max_health(),
            10,
            "Max health should remain unchanged"
        );
    }

    #[test]
    fn test_increase_health_to_max() {
        let mut health = PlayerHealth {
            current_health: 7,
            max_health: 10,
        };
        health.increase_health(3);
        assert_eq!(
            health.current_health(),
            10,
            "Health should increase exactly to max"
        );
    }

    #[test]
    fn test_increase_health_capped_at_max() {
        let mut health = PlayerHealth {
            current_health: 8,
            max_health: 10,
        };
        health.increase_health(5);
        assert_eq!(
            health.current_health(),
            10,
            "Health should be capped at max"
        );
    }

    #[test]
    fn test_increase_health_when_at_max() {
        let mut health = PlayerHealth {
            current_health: 10,
            max_health: 10,
        };
        health.increase_health(2);
        assert_eq!(health.current_health(), 10, "Health should remain at max");
    }

    #[test]
    fn test_increase_health_with_zero_amount() {
        let mut health = PlayerHealth {
            current_health: 5,
            max_health: 10,
        };
        health.increase_health(0);
        assert_eq!(
            health.current_health(),
            5,
            "Increasing by zero should have no effect"
        );
    }

    #[test]
    fn test_decrease_health_normal() {
        let mut health = PlayerHealth {
            current_health: 8,
            max_health: 10,
        };
        health.decrease_health(3);
        assert_eq!(
            health.current_health(),
            5,
            "Health should decrease normally"
        );
        assert_eq!(
            health.max_health(),
            10,
            "Max health should remain unchanged"
        );
    }

    #[test]
    fn test_decrease_health_to_zero() {
        let mut health = PlayerHealth {
            current_health: 5,
            max_health: 10,
        };
        health.decrease_health(5);
        assert_eq!(
            health.current_health(),
            0,
            "Health should decrease exactly to zero"
        );
    }

    #[test]
    fn test_decrease_health_stops_at_zero() {
        let mut health = PlayerHealth {
            current_health: 3,
            max_health: 10,
        };
        health.decrease_health(10);
        assert_eq!(health.current_health(), 0, "Health should stop at zero");
    }

    #[test]
    fn test_decrease_health_from_zero() {
        let mut health = PlayerHealth {
            current_health: 0,
            max_health: 10,
        };
        health.decrease_health(5);
        assert_eq!(health.current_health(), 0, "Health should remain at zero");
    }

    #[test]
    fn test_decrease_health_with_zero_amount() {
        let mut health = PlayerHealth {
            current_health: 5,
            max_health: 10,
        };
        health.decrease_health(0);
        assert_eq!(
            health.current_health(),
            5,
            "Decreasing by zero should have no effect"
        );
    }

    #[test]
    fn test_increase_max_health() {
        let mut health = PlayerHealth {
            current_health: 5,
            max_health: 10,
        };
        health.increase_max_health(5);
        assert_eq!(health.max_health(), 15, "Max health should increase");
        assert_eq!(
            health.current_health(),
            5,
            "Current health should remain unchanged"
        );
    }

    #[test]
    fn test_decrease_max_health_normal_no_current_change() {
        let mut health = PlayerHealth {
            current_health: 5,
            max_health: 10,
        };
        health.decrease_max_health(3);
        assert_eq!(
            health.max_health(),
            7,
            "Max health should decrease normally"
        );
        assert_eq!(
            health.current_health(),
            5,
            "Current health should be unaffected"
        );
    }

    #[test]
    fn test_decrease_max_health_reduces_current() {
        let mut health = PlayerHealth {
            current_health: 8,
            max_health: 10,
        };
        health.decrease_max_health(4);
        assert_eq!(health.max_health(), 6, "Max health should decrease");
        assert_eq!(
            health.current_health(),
            6,
            "Current health should be clamped to new max"
        );
    }

    #[test]
    fn test_decrease_max_health_to_zero() {
        let mut health = PlayerHealth {
            current_health: 5,
            max_health: 10,
        };
        health.decrease_max_health(15);
        assert_eq!(health.max_health(), 0, "Max health should decrease to zero");
        assert_eq!(
            health.current_health(),
            0,
            "Current health should be clamped to zero"
        );
    }

    #[test]
    fn test_decrease_max_health_stops_at_zero() {
        let mut health = PlayerHealth {
            current_health: 5,
            max_health: 3,
        };
        health.decrease_max_health(10);
        assert_eq!(
            health.max_health(),
            0,
            "Max health should stop decreasing at zero"
        );
        assert_eq!(
            health.current_health(),
            0,
            "Current health should also clamp to zero"
        );
    }

    #[test]
    fn test_getters() {
        let health = PlayerHealth {
            current_health: 7,
            max_health: 12,
        };
        assert_eq!(health.current_health(), 7);
        assert_eq!(health.max_health(), 12);
    }
}
