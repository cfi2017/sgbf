<script setup lang="ts">

</script>

<template>
  <v-card>
    <v-card-title>Pre-flight Checklist</v-card-title>
    <v-card-item>
      <v-expansion-panels>
        <v-expansion-panel>
          <v-expansion-panel-title>Preflight</v-expansion-panel-title>
          <v-expansion-panel-text>
            <v-list density="compact">
              <v-list-item>
                <v-list-item-title>Correct weights?</v-list-item-title>
                <p>Please consult your gliders weight chart. The chart shows the maximum, slightly below is usually better.</p>
              </v-list-item>
              <v-list-item>Tail dolly removed?</v-list-item>
            </v-list>
          </v-expansion-panel-text>
        </v-expansion-panel>
        <v-expansion-panel>
          <v-expansion-panel-title>Pre take-off</v-expansion-panel-title>
          <v-expansion-panel-text>
            <v-list density="compact">
              <v-list-item>Seatbelts fastened?</v-list-item>
              <v-list-item>
                <v-list-item-title>Instruments</v-list-item-title>
                <v-list density="compact">
                  <v-list-item>
                    <v-list-item-title>Speedometer</v-list-item-title>
                    <p>The speedometer should be at 0 before take-off.</p>
                  </v-list-item>
                  <v-list-item>
                    <v-list-item-title>LX</v-list-item-title>
                    <p>LX should be enabled and set to the correct altitude.</p>
                  </v-list-item>
                  <v-list-item>
                    <v-list-item-title>Varios</v-list-item-title>
                    <p>Varios should be at 0, unless you're lighter than air :)</p>
                  </v-list-item>
                  <v-list-item>
                    <v-list-item-title>Altimeter</v-list-item-title>
                    <p>Altimeter should show the correct altitude (545m for LSZI)</p>
                  </v-list-item>
                  <v-list-item>
                    <v-list-item-title>Accelerometer</v-list-item-title>
                    <p>Accelerometer should show G (1)</p>
                  </v-list-item>
                  <v-list-item>
                    <v-list-item-title>Transponder</v-list-item-title>
                    <p>Transponder should be enabled with ID 7000 and set to ALT.</p>
                  </v-list-item>
                  <v-list-item>
                    <v-list-item-title>Radio</v-list-item-title>
                    <p>Radio should be set to frequency 119.555.</p>
                  </v-list-item>
                  <v-list-item>
                    <v-list-item-title>FLARM</v-list-item-title>
                    <p>FLARM should be enabled and set to Mode 2 (collision warnings only).</p>
                  </v-list-item>
                  <v-list-item>
                    <v-list-item-title>ACL</v-list-item-title>
                    <p>Anti-Collision-Light should be switched on.</p>
                  </v-list-item>
                </v-list>
              </v-list-item>
              <v-list-item>Trim in starting position?</v-list-item>
              <v-list-item>Brake flaps functioning, secured?</v-list-item>
              <v-list-item>Vaulting in starting position?</v-list-item>
              <v-list-item>Steering free?</v-list-item>
              <v-list-item>Cockpit closed and secured?</v-list-item>
            </v-list>
          </v-expansion-panel-text>
        </v-expansion-panel>
        <v-expansion-panel>
          <v-expansion-panel-title>Departure Briefing</v-expansion-panel-title>
          <v-expansion-panel-text>
            <v-list density="compact">
              <v-list-item>
                <v-list-item-title>Wind</v-list-item-title>
                <p>Note strong winds and impact to flight.</p>
              </v-list-item>
              <v-list-item>Who's flying?</v-list-item>
              <v-list-item>Security: Take-off Speed</v-list-item>
              <v-list-item>Security: Height for turn-around</v-list-item>
              <v-list-item>
                <v-list-item-title>Security: Emergency Exit</v-list-item-title>
                <p>
                  Open cockpit first, then undo seatbelt.
                  Exit however necessary, if possible dodge the tail.
                  Wait ~2 seconds, then pull parachute.
                </p>
              </v-list-item>
              <v-list-item>Special conditions?</v-list-item>
            </v-list>
          </v-expansion-panel-text>
        </v-expansion-panel>
      </v-expansion-panels>
    </v-card-item>
  </v-card>
</template>

<style scoped>
p {
  font-size: 13px;
  color: grey;
}
</style>