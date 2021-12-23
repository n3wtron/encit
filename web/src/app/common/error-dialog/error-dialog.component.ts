import { Component, Inject, OnInit } from '@angular/core';
import { ErrorService } from '../error.service';
import { MAT_DIALOG_DATA } from '@angular/material/dialog';

@Component({
  selector: 'app-error-dialog',
  templateUrl: './error-dialog.component.html',
  styleUrls: ['./error-dialog.component.scss'],
})
export class ErrorDialogComponent implements OnInit {
  errorMessage: string | undefined;

  constructor(
    public readonly errorService: ErrorService,
    @Inject(MAT_DIALOG_DATA) public data: { error: string }
  ) {}

  ngOnInit(): void {
    this.errorMessage = this.data.error;
  }
}
