model pendel
  inner PlanarMechanics.PlanarWorld planarWorld annotation(
    Placement(transformation(origin = {52, 64}, extent = {{-10, -10}, {10, 10}})));
  PlanarMechanics.Joints.Revolute revolute(useFlange = true)  annotation(
    Placement(transformation(origin = {0, 22}, extent = {{10, 10}, {-10, -10}}, rotation = 90)));
  PlanarMechanics.Parts.FixedTranslation fixedTranslation annotation(
    Placement(transformation(origin = {0, -6}, extent = {{-10, -10}, {10, 10}}, rotation = -90)));
  PlanarMechanics.Parts.Body body(m = 1, I = 0)  annotation(
    Placement(transformation(origin = {0, -34}, extent = {{-10, -10}, {10, 10}}, rotation = -90)));
  PlanarMechanics.Parts.Fixed fixed annotation(
    Placement(transformation(origin = {0, 52}, extent = {{-10, -10}, {10, 10}}, rotation = 90)));
  Modelica.Mechanics.Rotational.Components.Damper damper(d = .3)  annotation(
    Placement(transformation(origin = {-17, 25}, extent = {{-3, -3}, {3, 3}}, rotation = -90)));
equation
  connect(fixed.frame, revolute.frame_a) annotation(
    Line(points = {{0, 42}, {0, 32}}, color = {95, 95, 95}));
  connect(revolute.frame_b, fixedTranslation.frame_a) annotation(
    Line(points = {{0, 12}, {0, 4}}, color = {95, 95, 95}));
  connect(fixedTranslation.frame_b, body.frame_a) annotation(
    Line(points = {{0, -16}, {0, -24}}, color = {95, 95, 95}));
  connect(damper.flange_a, revolute.support) annotation(
    Line(points = {{-16, 28}, {-10, 28}}));
  connect(revolute.flange_a, damper.flange_b) annotation(
    Line(points = {{-10, 22}, {-16, 22}}));

annotation(
    uses(PlanarMechanics(version = "1.6.0"), Modelica(version = "4.1.0")),
  experiment(StartTime = 0, StopTime = 60, Tolerance = 1e-06, Interval = 0.12));
end pendel;