// ==========================================
// 1. STRUKTUREN (Datenaustausch)
// ==========================================

// Was kommt aus unserem Vertex-Buffer (Grafikkarten-Speicher) rein?
struct VertexInput {
    @location(0) position: vec3<f32>, // X, Y, Z Koordinaten im Raum
    @location(1) color: vec3<f32>,    // R, G, B Farbe des Punktes
};

// Was gibt der Vertex-Shader an den Fragment-Shader weiter?
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>, // Die endgültige Position auf dem Bildschirm
    @location(0) color: vec3<f32>,             // Die interpolierte Farbe für den Pixel
};

// ==========================================
// 2. VERTEX SHADER
// ==========================================
// Aufgabe: Berechnet, *wo* im Raum die Eckpunkte liegen.
@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    // Farbe einfach 1-zu-1 an den Pixel-Shader weiterreichen
    out.color = model.color;

    // Position von 3D (x, y, z) in ein 4D-Clipping-Format (x, y, z, w) umwandeln
    out.clip_position = vec4<f32>(model.position, 1.0);

    return out;
}

// ==========================================
// 3. FRAGMENT SHADER (Pixel Shader)
// ==========================================
// Aufgabe: Bestimmt die exakte *Farbe* für jeden einzelnen Pixel zwischen den Eckpunkten.
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Gibt die Farbe (mit voller Alpha-/Deckkraft-Transparenz 1.0) an den Bildschirm zurück
    return vec4<f32>(in.color, 1.0);
}