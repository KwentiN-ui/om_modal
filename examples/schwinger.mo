model schwinger
  Modelica.Mechanics.Translational.Components.Fixed fixed annotation(
    Placement(transformation(origin = {-20, 0}, extent = {{-10, -10}, {10, 10}}, rotation = -90)));
  Modelica.Mechanics.Translational.Components.Mass mass(m = 1)  annotation(
    Placement(transformation(origin = {30, 0}, extent = {{-10, -10}, {10, 10}})));
  Modelica.Mechanics.Translational.Components.SpringDamper springDamper(c = 1, d = 0.1)  annotation(
    Placement(transformation(extent = {{-10, -10}, {10, 10}})));
equation
  connect(fixed.flange, springDamper.flange_a) annotation(
    Line(points = {{-20, 0}, {-10, 0}}, color = {0, 127, 0}));
  connect(springDamper.flange_b, mass.flange_a) annotation(
    Line(points = {{10, 0}, {20, 0}}, color = {0, 127, 0}));
  annotation(
    uses(Modelica(version = "4.1.0")));
end schwinger;