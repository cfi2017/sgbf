{{/*
Expand the name of the chart.
*/}}
{{- define "sgbf.name" -}}
{{- default .Chart.Name .Values.nameOverride | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Create a default fully qualified app name.
*/}}
{{- define "sgbf.fullname" -}}
{{- if .Values.fullnameOverride }}
{{- .Values.fullnameOverride | trunc 63 | trimSuffix "-" }}
{{- else }}
{{- $name := default .Chart.Name .Values.nameOverride }}
{{- if contains $name .Release.Name }}
{{- .Release.Name | trunc 63 | trimSuffix "-" }}
{{- else }}
{{- printf "%s-%s" .Release.Name $name | trunc 63 | trimSuffix "-" }}
{{- end }}
{{- end }}
{{- end }}

{{/*
Create chart name and version as used by the chart label.
*/}}
{{- define "sgbf.chart" -}}
{{- printf "%s-%s" .Chart.Name .Chart.Version | replace "+" "_" | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Common labels
*/}}
{{- define "sgbf.labels" -}}
helm.sh/chart: {{ include "sgbf.chart" . }}
{{ include "sgbf.selectorLabels" . }}
{{- if .Chart.AppVersion }}
app.kubernetes.io/version: {{ .Chart.AppVersion | quote }}
{{- end }}
app.kubernetes.io/managed-by: {{ .Release.Service }}
{{- end }}

{{/*
Selector labels
*/}}
{{- define "sgbf.selectorLabels" -}}
app.kubernetes.io/name: {{ include "sgbf.name" . }}
app.kubernetes.io/instance: {{ .Release.Name }}
{{- end }}

{{/*
API labels
*/}}
{{- define "sgbf.api.labels" -}}
{{ include "sgbf.labels" . }}
app.kubernetes.io/component: api
app: {{ .Values.api.name }}
{{- with .Values.api.labels }}
{{ toYaml . }}
{{- end }}
{{- end }}

{{/*
API selector labels
*/}}
{{- define "sgbf.api.selectorLabels" -}}
{{ include "sgbf.selectorLabels" . }}
app: {{ .Values.api.name }}
{{- end }}

{{/*
Frontend labels
*/}}
{{- define "sgbf.frontend.labels" -}}
{{ include "sgbf.labels" . }}
app.kubernetes.io/component: frontend
app: {{ .Values.frontend.name }}
{{- with .Values.frontend.labels }}
{{ toYaml . }}
{{- end }}
{{- end }}

{{/*
Frontend selector labels
*/}}
{{- define "sgbf.frontend.selectorLabels" -}}
{{ include "sgbf.selectorLabels" . }}
app: {{ .Values.frontend.name }}
{{- end }}

{{/*
Linkerd annotations
*/}}
{{- define "sgbf.linkerdAnnotations" -}}
{{- if .Values.global.linkerd.enabled }}
linkerd.io/inject: enabled
{{- end }}
{{- end }}
