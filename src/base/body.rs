use crate::engine::{Engine, Plugin};
use hecs::Entity;
use raylib::prelude::*;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Default)]
pub struct BodyPlugin;
impl Plugin for BodyPlugin {
    fn add_plugin(
        engine: &mut crate::engine::Engine,
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
    ) {
        engine.add_update(Body::update).add_draw(Body::draw);
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Body {
    pub pos: Vector2,
    pub vel: Vector2,
    pub rot: f32,
    pub torque: f32,
    pub shape: CollisionShape,
    pub properties: BodyProps,
    pub parent: Option<Entity>,
}
#[derive(Debug, Clone, PartialEq)]
pub enum CollisionShape {
    Rect(Vector2),
    Circle(f32),
}
#[derive(Debug, Clone, PartialEq)]
pub struct BodyProps {
    pub temp: f32,
    pub press: f32,
    pub mass: f32,
    pub mats: usize,
    pub elems: HashSet<Element>,
}
impl Default for BodyProps {
    fn default() -> Self {
        Self {
            temp: 0.0,
            press: 0.0,
            mass: 1.0,
            mats: 0,
            elems: HashSet::default(),
        }
    }
}
impl Body {
    #[inline(always)]
    pub fn step(&mut self, dt: f32) {
        self.pos += self.vel * dt;
        self.rot += self.torque * dt;
    }
    #[inline(always)]
    pub fn collision(&self, other: &Self) -> bool {
        match self.shape {
            CollisionShape::Rect(size1) => {
                let a = Rectangle {
                    x: self.pos.x,
                    y: self.pos.y,
                    width: size1.x,
                    height: size1.y,
                };
                match other.shape {
                    CollisionShape::Rect(size2) => {
                        let b = Rectangle {
                            x: other.pos.x,
                            y: other.pos.y,
                            width: size2.x,
                            height: size2.y,
                        };
                        a.check_collision_recs(&b)
                    }
                    CollisionShape::Circle(rad2) => a.check_collision_circle_rec(other.pos, rad2),
                    _ => false,
                }
            }
            CollisionShape::Circle(rad1) => match other.shape {
                CollisionShape::Rect(size2) => {
                    let b = Rectangle {
                        x: other.pos.x,
                        y: other.pos.y,
                        width: size2.x,
                        height: size2.y,
                    };
                    b.check_collision_circle_rec(self.pos, rad1)
                }
                CollisionShape::Circle(rad2) => {
                    check_collision_circles(self.pos, rad1, other.pos, rad2)
                }
                _ => false,
            },
        }
    }
    #[inline(always)]
    pub fn update(engine: &mut Engine, _: (&mut RaylibHandle, &mut RaylibThread), dt: f32) {
        let mut snaps = vec![];
        for (e, body) in engine.world.query_mut::<(Entity, &mut Body)>() {
            snaps.push((e, body));
        }

        for i in 0..snaps.len() {
            for j in (i + 1)..snaps.len() {
                let (left, right) = snaps.split_at_mut(j);
                let (_e_a, a) = &mut left[i];
                let (_e_b, b) = &mut right[0];

                if let (CollisionShape::Circle(rad_a), CollisionShape::Circle(rad_b)) =
                    (&a.shape, &b.shape)
                {
                    let delta = b.pos - a.pos;
                    let dist = delta.length().max(0.0001);
                    let penetration = rad_a + rad_b - dist;
                    if penetration <= 0.0 {
                        continue;
                    }

                    let normal = delta / dist;
                    let rel_vel = b.vel - a.vel;
                    let vel_along = rel_vel.dot(normal);
                    if vel_along > 0.0 {
                        // moving apart
                    } else {
                        let inv_mass_a = 1.0 / a.properties.mass.max(0.0001);
                        let inv_mass_b = 1.0 / b.properties.mass.max(0.0001);
                        let restitution = 0.9; // 1.0 perfectly elastic, <1 for energy loss
                        let j = -(1.0 + restitution) * vel_along / (inv_mass_a + inv_mass_b);
                        let impulse = normal * j;
                        a.vel -= impulse * inv_mass_a;
                        b.vel += impulse * inv_mass_b;
                    }

                    // penetration correction (prevent sinking)
                    let inv_mass_a = 1.0 / a.properties.mass.max(0.0001);
                    let inv_mass_b = 1.0 / b.properties.mass.max(0.0001);
                    let percent = 0.8;
                    let slop = 0.01;
                    let correction = normal * ((penetration - slop).max(0.0))
                        / (inv_mass_a + inv_mass_b)
                        * percent;
                    a.pos -= correction * inv_mass_a;
                    b.pos += correction * inv_mass_b;
                }
            }
        }

        for (_e, body) in snaps {
            body.step(dt);
        }
    }
    #[inline(always)]
    pub fn draw(engine: &mut Engine, (d, _): (&mut RaylibDrawHandle, &mut RaylibThread)) {
        for body in engine.world.query_mut::<&mut Body>() {
            match body.shape {
                CollisionShape::Rect(size) => {
                    d.draw_rectangle_lines(
                        body.pos.x as i32,
                        body.pos.y as i32,
                        size.x as i32,
                        size.y as i32,
                        Color::WHITE,
                    );
                }
                CollisionShape::Circle(rad) => {
                    d.draw_circle_lines_v(body.pos, rad, Color::WHITE);
                }
            }
            d.draw_line_v(body.pos, body.pos + body.vel * 0.1, Color::GREEN);
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Element {
    H,
    He,
    Li,
    Be,
    B,
    C,
    N,
    O,
    F,
    Ne,
    Na,
    Mg,
    Al,
    Si,
    P,
    S,
    Cl,
    Ar,
    K,
    Ca,
    Sc,
    Ti,
    V,
    Cr,
    Mn,
    Fe,
    Co,
    Ni,
    Cu,
    Zn,
    Ga,
    Ge,
    As,
    Se,
    Br,
    Kr,
    Rb,
    Sr,
    Y,
    Zr,
    Nb,
    Mo,
    Tc,
    Ru,
    Rh,
    Pd,
    Ag,
    Cd,
    In,
    Sn,
    Sb,
    Te,
    I,
    Xe,
    Cs,
    Ba,
    La,
    Ce,
    Pr,
    Nd,
    Pm,
    Sm,
    Eu,
    Gd,
    Tb,
    Dy,
    Ho,
    Er,
    Tm,
    Yb,
    Lu,
    Hf,
    Ta,
    W,
    Re,
    Os,
    Ir,
    Pt,
    Au,
    Hg,
    Tl,
    Pb,
    Bi,
    Po,
    At,
    Rn,
    Fr,
    Ra,
    Ac,
    Th,
    Pa,
    U,
    Np,
    Pu,
    Am,
    Cm,
    Bk,
    Cf,
    Es,
    Fm,
    Md,
    No,
    Lr,
    Rf,
    Db,
    Sg,
    Bh,
    Hs,
    Mt,
    Ds,
    Rg,
    Cn,
    Nh,
    Fl,
    Mc,
    Lv,
    Ts,
    Og,
}
pub enum ElementCategories {
    AlkaliMetal,
    Lanthanides,
    AlkalineEarthMetal,
    Actinide,
    TransitionMetal,
    Nonmetal,
    PostTransitionMetal,
    Halogen,
    Metalloid,
    NobleGas,
}
impl From<Element> for ElementCategories {
    fn from(val: Element) -> Self {
        use Element::*;
        use ElementCategories::*;
        match val {
            Li | Na | K | Rb | Cs | Fr => AlkaliMetal,
            La | Ce | Pr | Nd | Pm | Sm | Eu | Gd | Tb | Dy | Ho | Er | Tm | Yb | Lu => Lanthanides,
            Be | Mg | Ca | Sr | Ba | Ra => AlkalineEarthMetal,
            Ac | Th | Pa | U | Np | Pu | Am | Cm | Bk | Cf | Es | Fm | Md | No | Lr => Actinide,
            Sc | Ti | V | Cr | Mn | Fe | Co | Ni | Cu | Zn | Y | Zr | Nb | Mo | Tc | Ru | Rh
            | Pd | Ag | Cd | Hf | Ta | W | Re | Os | Ir | Pt | Au | Hg | Rf | Db | Sg | Bh | Hs
            | Mt | Ds | Rg | Cn => TransitionMetal,
            H | C | N | O | P | S | Se => Nonmetal,
            Al | Ga | In | Sn | Tl | Pb | Bi | Nh | Fl | Mc | Lv => PostTransitionMetal,
            F | Cl | Br | I | At | Ts => Halogen,
            B | Si | Ge | As | Sb | Te | Po => Metalloid,
            He | Ne | Ar | Kr | Xe | Rn | Og => NobleGas,
        }
    }
}
